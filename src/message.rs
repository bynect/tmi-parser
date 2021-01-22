//! IRC-based TMI messages.

use crate::{TagValue, Tags};
use std::fmt::Write;
use std::io::{Error, ErrorKind, Result};

/// Possible types of TMI messages.
/// Unrecognized messages are handled by the associated [`parse`] function.
///
/// Tags are always treated as Optional even on messages that require them.
/// Actually, tags validation should be done by the user code.
///
/// Consider changing simple enum structs to enum tuples.
#[derive(Debug, PartialEq)]
pub enum Message<'a> {
    /// Represents a ping request message.
    /// `PING :<endpoint>`
    Ping,
    /// Represents a pong response message.
    /// `PONG :<endpoint>`
    Pong,
    /// Represents a capability request message.
    /// `CAP REQ :<capability>`
    CapReq { req: &'a str },
    /// Represents a capability acknowledgement message.
    /// `:<endpoint> CAP * ACK :<capability>`
    CapAck { req: &'a str },
    /// Represents a password authentication message.
    /// `PASS <password>`
    /// `PASS oauth:<token>` (using Twitch OAuth tokens)
    Pass { pass: &'a str },
    /// Represents a nickname authentication message.
    /// `NICK <user>`
    Nick { nick: &'a str },
    /// Represents a join command message.
    /// `JOIN #<channel>`
    Join { chan: &'a str },
    /// Represents a part command message.
    /// `PART #<channel>`
    Part { chan: &'a str },
    /// Represents a privmsg command message.
    /// `[@<tags>] PRIVMSG #<channel> :<message>`
    Privmsg {
        tags: Option<Tags<'a>>,
        chan: &'a str,
        msg: &'a str,
    },
    /// Represents a clearchat command message.
    /// `[@<tags>] :<endpoint> CLEARCHAT #<channel> [:<user>]`
    Clearchat {
        tags: Option<Tags<'a>>,
        chan: &'a str,
        usr: Option<&'a str>,
    },
    /// Represents a clearmsg command message.
    /// `[@<tags>] :<endpoint> CLEARMSG #<channel> :<message>`
    Clearmsg {
        tags: Option<Tags<'a>>,
        chan: &'a str,
        msg: &'a str,
    },
    /// Represents a hosttarget start message.
    /// `:<endpoint> HOSTTARGET #<host> :<channel> [<viewers>]`
    HosttargetStart {
        host: &'a str,
        chan: &'a str,
        view: Option<u32>,
    },
    /// Represents a hosttarget end message.
    /// `:<endpoint> HOSTTARGET #<host> :- [<viewers>]`
    HosttargetEnd { host: &'a str, view: Option<u32> },
    /// Represents a notice message.
    /// `[@<tags>] :<endpoint> NOTICE #<channel> :<message>`
    Notice {
        tags: Option<Tags<'a>>,
        chan: &'a str,
        msg: &'a str,
    },
    /// Represents a reconnect request message.
    /// `RECONNECT`
    Reconnect,
    /// Represents a roomstate message.
    /// `[@<tags>] :<endpoint> ROOMSTATE #<channel>`
    Roomstate {
        tags: Option<Tags<'a>>,
        chan: &'a str,
    },
    /// Represents a usernotice message.
    /// `[@<tags>] :<endpoint> USERNOTICE #<channel> :<message>`
    Usernotice {
        tags: Option<Tags<'a>>,
        chan: &'a str,
        msg: &'a str,
    },
    /// Represents a userstate message.
    /// `[@<tags>] :<endpoint> USERSTATE #<channel>`
    Userstate {
        tags: Option<Tags<'a>>,
        chan: &'a str,
    },
    /// Represents a global userstate message.
    /// `[@<tags>] :<endpoint> GLOBALUSERSTATE`
    GlobalUserstate { tags: Option<Tags<'a>> },
}

impl<'a> Message<'a> {
    /// Parses a [`& str`] slice and returns a Message if successful, otherwise an [`std::io::Error`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use tmi_parser::*;
    /// let s = ":tmi.twitch.tv CLEARCHAT #dallas :ronni";
    /// let msg = Message::parse(s);
    /// ```
    pub fn parse(msg: &'a str) -> Result<Message> {
        if msg.len() < 5 {
            return Err(Error::new(ErrorKind::Other, "Malformed message."));
        }

        let buf = msg.trim();
        let (tags, off) = if let Some(buf) = buf.strip_prefix('@') {
            Self::parse_tags(buf)?
        } else {
            (None, 0)
        };

        let mut rest = &buf[off..];
        const ENDPOINT: &str = "tmi.twitch.tv ";

        if let Some(off) = rest.find(ENDPOINT) {
            rest = &rest[(off + ENDPOINT.len())..];
        }

        if let Some(off) = rest.find(' ') {
            let cmd = &rest[..off];
            let body = &rest[(off + 1)..];

            Self::parse_command(cmd, body, tags)
        } else {
            Self::parse_command(rest, "", tags)
        }
    }

    /// Helper function for parsing message tags.
    fn parse_tags(msg: &'a str) -> Result<(Option<Tags<'a>>, usize)> {
        let mut map = Tags::default();

        if let Some(idx) = msg.find(" :") {
            let tag = &msg[..idx];
            let toks = tag.split(';');

            for tok in toks {
                let items = tok.split('=').collect::<Vec<_>>();
                let key = items[0];
                let val = items[1];

                map.insert(key, TagValue::new(val));
            }

            Ok((Some(map), idx + 3))
        } else {
            Err(Error::new(ErrorKind::Other, "Parsing message tags failed."))
        }
    }

    /// Helper function for parsing message body base on the command.
    fn parse_command(cmd: &'a str, body: &'a str, tags: Option<Tags<'a>>) -> Result<Message<'a>> {
        Ok(match cmd {
            "PING" => Message::Ping,
            "PONG" => Message::Pong,
            "CAP" => {
                let off = body
                    .find(" :")
                    .ok_or_else(|| Error::new(ErrorKind::Other, "Malformed CAP command."))?;

                match &body[..off] {
                    "REQ" => Message::CapReq {
                        req: &body[(off + 2)..],
                    },
                    "* ACK" => Message::CapAck {
                        req: &body[(off + 2)..],
                    },
                    _ => return Err(Error::new(ErrorKind::Other, "Malformed CAP command.")),
                }
            }
            "PASS" => Message::Pass { pass: body },
            "NICK" => Message::Nick { nick: body },
            "JOIN" => Message::Join { chan: &body[1..] },
            "PART" => Message::Part { chan: &body[1..] },
            "PRIVMSG" => {
                let off = body
                    .find(" :")
                    .ok_or_else(|| Error::new(ErrorKind::Other, "Malformed PRIVMSG command."))?;

                Message::Privmsg {
                    tags,
                    chan: &body[1..off],
                    msg: &body[(off + 2)..],
                }
            }
            "CLEARCHAT" => {
                if let Some(off) = body.find(" :") {
                    Message::Clearchat {
                        tags,
                        chan: &body[1..off],
                        usr: Some(&body[(off + 2)..]),
                    }
                } else {
                    Message::Clearchat {
                        tags,
                        chan: &body[1..],
                        usr: None,
                    }
                }
            }
            "CLEARMSG" => {
                let off = body
                    .find(" :")
                    .ok_or_else(|| Error::new(ErrorKind::Other, "Malformed CLEARMSG command."))?;

                Message::Clearmsg {
                    tags,
                    chan: &body[1..off],
                    msg: &body[(off + 2)..],
                }
            }
            "HOSTTARGET" => {
                if let Some(off) = body.find(" :-") {
                    if body.len() > off + 5 {
                        if let Ok(view) = body[(off + 4)..].parse::<u32>() {
                            Message::HosttargetEnd {
                                host: &body[1..off],
                                view: Some(view),
                            }
                        } else {
                            return Err(Error::new(
                                ErrorKind::Other,
                                "Malformed HOSTTARGET command.",
                            ));
                        }
                    } else {
                        Message::HosttargetEnd {
                            host: &body[1..off],
                            view: None,
                        }
                    }
                } else {
                    let off = body.find(" :").ok_or_else(|| {
                        Error::new(ErrorKind::Other, "Malformed HOSTTARGET command.")
                    })?;

                    if body.len() < off + 3 {
                        return Err(Error::new(
                            ErrorKind::Other,
                            "Malformed HOSTTARGET command.",
                        ));
                    }

                    let host = &body[1..off];
                    let body = &body[(off + 2)..];

                    if let Some(off) = body.find(' ') {
                        if body.len() > off + 2 {
                            if let Ok(view) = body[(off + 1)..].parse::<u32>() {
                                return Ok(Message::HosttargetStart {
                                    host,
                                    chan: &body[..off],
                                    view: Some(view),
                                });
                            } else {
                                return Err(Error::new(
                                    ErrorKind::Other,
                                    "Malformed HOSTTARGET command.",
                                ));
                            }
                        } else {
                            Message::HosttargetStart {
                                host,
                                chan: body,
                                view: None,
                            }
                        }
                    } else {
                        Message::HosttargetStart {
                            host,
                            chan: body,
                            view: None,
                        }
                    }
                }
            }
            "NOTICE" => {
                let off = body
                    .find(" :")
                    .ok_or_else(|| Error::new(ErrorKind::Other, "Malformed NOTICE command."))?;

                Message::Notice {
                    tags,
                    chan: &body[1..off],
                    msg: &body[(off + 2)..],
                }
            }
            "RECONNECT" => Message::Reconnect,
            "ROOMSTATE" => Message::Roomstate {
                tags,
                chan: &body[1..],
            },
            "USERNOTICE" => {
                let off = body
                    .find(" :")
                    .ok_or_else(|| Error::new(ErrorKind::Other, "Malformed USERNOTICE command."))?;

                Message::Usernotice {
                    tags,
                    chan: &body[1..off],
                    msg: &body[(off + 2)..],
                }
            }
            "USERSTATE" => Message::Userstate {
                tags,
                chan: &body[1..],
            },
            "GLOBALUSERSTATE" => Message::GlobalUserstate { tags },
            _ => {
                return Err(Error::new(
                    ErrorKind::Other,
                    "Parsing message command failed.",
                ))
            }
        })
    }

    /// Unparses a Message and returns a newly allocated [`String`] if successful, otherwise an [`std::io::Error`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use tmi_parser::*;
    /// let msg = Message::Part { chan: "dallas" };
    /// let s = msg.unparse();
    /// ```
    pub fn unparse(&self) -> Result<String> {
        const ENDPOINT: &str = "tmi.twitch.tv";

        Ok(match self {
            Message::Ping => format!("PING {}", ENDPOINT),
            Message::Pong => format!("PONG {}", ENDPOINT),
            Message::CapReq { req } => format!("CAP REQ {}", req),
            Message::CapAck { req } => format!("{} CAP * ACK {}", ENDPOINT, req),
            Message::Pass { pass } => format!("PASS {}", pass),
            Message::Nick { nick } => format!("NICK {}", nick),
            Message::Join { chan } => format!("JOIN #{}", chan),
            Message::Part { chan } => format!("PART #{}", chan),
            Message::Privmsg { tags, chan, msg } => {
                if let Some(mut body) = Self::unparse_tags(tags) {
                    write!(body, "PRIVMSG #{} :{}", chan, msg).unwrap();
                    body
                } else {
                    format!("PRIVMSG #{} :{}", chan, msg)
                }
            }
            Message::Clearchat { tags, chan, usr } => {
                let mut body = match Self::unparse_tags(tags) {
                    Some(body) => body,
                    _ => String::new(),
                };

                if let Some(usr) = usr {
                    write!(body, ":{} CLEARCHAT #{} :{}", ENDPOINT, chan, usr).unwrap();
                } else {
                    write!(body, ":{} CLEARCHAT #{}", ENDPOINT, chan).unwrap();
                }

                body
            }
            Message::Clearmsg { tags, chan, msg } => {
                if let Some(mut body) = Self::unparse_tags(tags) {
                    write!(body, ":{} CLEARMSG #{} :{}", ENDPOINT, chan, msg).unwrap();
                    body
                } else {
                    format!(":{} CLEARMSG #{} :{}", ENDPOINT, chan, msg)
                }
            }
            Message::HosttargetStart { host, chan, view } => {
                if let Some(view) = view {
                    format!(":{} HOSTTARGET #{} :{} {}", ENDPOINT, host, chan, view)
                } else {
                    format!(":{} HOSTTARGET #{} :{}", ENDPOINT, host, chan)
                }
            }
            Message::HosttargetEnd { host, view } => {
                if let Some(view) = view {
                    format!(":{} HOSTTARGET #{} :- {}", ENDPOINT, host, view)
                } else {
                    format!(":{} HOSTTARGET #{} :-", ENDPOINT, host)
                }
            }
            Message::Notice { tags, chan, msg } => {
                if let Some(mut body) = Self::unparse_tags(tags) {
                    write!(body, ":{} NOTICE #{} :{}", ENDPOINT, chan, msg).unwrap();
                    body
                } else {
                    format!(":{} NOTICE #{} :{}", ENDPOINT, chan, msg)
                }
            }
            Message::Reconnect => String::from("RECONNECT"),
            Message::Roomstate { tags, chan } => {
                if let Some(mut body) = Self::unparse_tags(tags) {
                    write!(body, ":{} ROOMSTATE #{}", ENDPOINT, chan).unwrap();
                    body
                } else {
                    format!(":{} ROOMSTATE #{}", ENDPOINT, chan)
                }
            }
            Message::Usernotice { tags, chan, msg } => {
                if let Some(mut body) = Self::unparse_tags(tags) {
                    write!(body, ":{} USERNOTICE #{} :{}", ENDPOINT, chan, msg).unwrap();
                    body
                } else {
                    format!(":{} USERNOTICE #{} :{}", ENDPOINT, chan, msg)
                }
            }
            Message::Userstate { tags, chan } => {
                if let Some(mut body) = Self::unparse_tags(tags) {
                    write!(body, ":{} USERSTATE #{}", ENDPOINT, chan).unwrap();
                    body
                } else {
                    format!(":{} USERSTATE #{}", ENDPOINT, chan)
                }
            }
            Message::GlobalUserstate { tags } => {
                if let Some(mut body) = Self::unparse_tags(tags) {
                    write!(body, ":{} GLOBALUSERSTATE", ENDPOINT).unwrap();
                    body
                } else {
                    format!(":{} GLOBALUSERSTATE", ENDPOINT)
                }
            }
        })
    }

    /// Helper function for unparsing message tags.
    /// [`Tags`] doesn't store the original order of items!
    fn unparse_tags(tags: &Option<Tags<'a>>) -> Option<String> {
        if let Some(tags) = tags {
            if !tags.is_empty() {
                let mut raw = String::from("@");

                for (key, val) in tags {
                    raw.push_str(key);
                    raw.push('=');
                    raw.push_str(&format!("{}", val));
                    raw.push(';');
                }

                raw.pop();
                raw.push(' ');

                Some(raw)
            } else {
                None
            }
        } else {
            None
        }
    }
}
