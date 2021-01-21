use std::collections::HashMap;
use std::io::{Error, ErrorKind, Result};

pub type Tags<'a> = HashMap<&'a str, TagValue<'a>>;

#[derive(Debug, PartialEq)]
pub enum TagValue<'a> {
    Number(u32),
    Timestamp(u64),
    Boolean(bool),
    String(&'a str),
    Error(&'a str),
    None,
}

#[derive(Debug, PartialEq)]
pub enum Message<'a> {
    CapReq {
        req: &'a str,
    },
    CapAck {
        req: &'a str,
    },
    Pass {
        pass: &'a str,
    },
    Nick {
        nick: &'a str,
    },
    Join {
        chan: &'a str,
    },
    Part {
        chan: &'a str,
    },
    Privmsg {
        tags: Option<Tags<'a>>,
        chan: &'a str,
        msg: &'a str,
    },
    Clearchat {
        tags: Option<Tags<'a>>,
        chan: &'a str,
        usr: Option<&'a str>,
    },
    Clearmsg {
        tags: Option<Tags<'a>>,
        chan: &'a str,
        msg: &'a str,
    },
    HosttargetStart {
        host: &'a str,
        chan: &'a str,
        view: Option<u32>,
    },
    HosttargetEnd {
        host: &'a str,
        view: Option<u32>,
    },
    Notice {
        tags: Option<Tags<'a>>,
        chan: &'a str,
        msg: &'a str,
    },
    Reconnect,
    Roomstate {
        tags: Option<Tags<'a>>,
        chan: &'a str,
    },
    Usernotice {
        tags: Option<Tags<'a>>,
        chan: &'a str,
        msg: &'a str,
    },
    Userstate {
        tags: Option<Tags<'a>>,
        chan: &'a str,
    },
    GlobalUserstate {
        tags: Option<Tags<'a>>,
    },
}

impl<'a> Message<'a> {
    pub fn parse(msg: &'a str) -> Result<Message> {
        let buf = if msg.ends_with("\r\n") {
            &msg[..(msg.len() - 2)]
        } else {
            msg
        };

        let (tags, off) = if buf.starts_with('@') {
            let buf = &buf[1..];

            Self::parse_tags(buf)?
        } else {
            (None, 0)
        };

        let mut rest = &buf[off..];
        const ENDPOINT: &str = "tmi.twitch.tv ";

        if let Some(off) = rest.find(ENDPOINT) {
            rest = &rest[(off + ENDPOINT.len())..];
        }

        let off = rest
            .find(' ')
            .ok_or(Error::new(ErrorKind::Other, "Parsing message body failed."))?;

        let cmd = &rest[..off];
        let body = &rest[(off + 1)..];

        Self::parse_command(cmd, body, tags)
    }

    fn parse_tags(msg: &'a str) -> Result<(Option<Tags<'a>>, usize)> {
        let mut map = Tags::new();

        if let Some(idx) = msg.find(" :") {
            let tag = &msg[..idx];
            let toks = tag.split(';');

            for tok in toks {
                let items = tok.split('=').collect::<Vec<_>>();
                let key = items[0];
                let val = items[1];

                map.insert(
                    key,
                    match val {
                        "" | " " => TagValue::None,
                        "0" => TagValue::Boolean(false),
                        "1" => TagValue::Boolean(true),
                        _ => {
                            if let Ok(num) = val.parse::<u32>() {
                                TagValue::Number(num)
                            } else if let Ok(tm) = val.parse::<u64>() {
                                TagValue::Timestamp(tm)
                            } else if items[1].is_ascii() {
                                TagValue::String(val)
                            } else {
                                TagValue::Error(val)
                            }
                        }
                    },
                );
            }

            Ok((Some(map), idx + 3))
        } else {
            Err(Error::new(ErrorKind::Other, "Parsing message tags failed."))
        }
    }

    fn parse_command(cmd: &'a str, body: &'a str, tags: Option<Tags<'a>>) -> Result<Message<'a>> {
        Ok(match cmd {
            "CAP" => {
                let off = body
                    .find(" :")
                    .ok_or(Error::new(ErrorKind::Other, "Malformed CAP command."))?;

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
                    .ok_or(Error::new(ErrorKind::Other, "Malformed PRIVMSG command."))?;

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
                    .ok_or(Error::new(ErrorKind::Other, "Malformed CLEARMSG command."))?;

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
                    let off = body.find(" :").ok_or(Error::new(
                        ErrorKind::Other,
                        "Malformed HOSTTARGET command.",
                    ))?;

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
                    .ok_or(Error::new(ErrorKind::Other, "Malformed NOTICE command."))?;

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
                let off = body.find(" :").ok_or(Error::new(
                    ErrorKind::Other,
                    "Malformed USERNOTICE command.",
                ))?;

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
}
