use std::io::Result;
use tmi_parser::*;

#[test]
fn parse_capreq() -> Result<()> {
    let cap1 = "CAP REQ :twitch.tv/membership";
    let cap2 = "CAP REQ :twitch.tv/tags\r\n";

    assert_eq!(
        Message::parse(cap1)?,
        Message::CapReq {
            req: "twitch.tv/membership"
        }
    );

    assert_eq!(
        Message::parse(cap2)?,
        Message::CapReq {
            req: "twitch.tv/tags"
        }
    );

    Ok(())
}

#[test]
fn parse_capack() -> Result<()> {
    let cap1 = ":tmi.twitch.tv CAP * ACK :twitch.tv/commands";
    let cap2 = ":tmi.twitch.tv CAP * ACK :twitch.tv/tags\r\n";

    assert_eq!(
        Message::parse(cap1)?,
        Message::CapAck {
            req: "twitch.tv/commands"
        }
    );

    assert_eq!(
        Message::parse(cap2)?,
        Message::CapAck {
            req: "twitch.tv/tags"
        }
    );

    Ok(())
}
