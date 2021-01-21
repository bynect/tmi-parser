use std::io::Result;
use tmi_parser::*;

#[test]
fn parse_hosttarget_start() -> Result<()> {
    let host1 = ":tmi.twitch.tv HOSTTARGET #hosting_channel :<channel>";
    let host2 = "tmi.twitch.tv HOSTTARGET #hosting_channel :<channel> 123456";

    assert_eq!(
        Message::parse(host1)?,
        Message::HosttargetStart {
            host: "hosting_channel",
            chan: "<channel>",
            view: None,
        }
    );

    assert_eq!(
        Message::parse(host2)?,
        Message::HosttargetStart {
            host: "hosting_channel",
            chan: "<channel>",
            view: Some(123456u32),
        }
    );

    Ok(())
}

#[test]
fn parse_hosttarget_end() -> Result<()> {
    let host1 = ":tmi.twitch.tv HOSTTARGET #hosting_channel :-";
    let host2 = ":tmi.twitch.tv HOSTTARGET #hosting_channel :- 123456";

    assert_eq!(
        Message::parse(host1)?,
        Message::HosttargetEnd {
            host: "hosting_channel",
            view: None,
        }
    );

    assert_eq!(
        Message::parse(host2)?,
        Message::HosttargetEnd {
            host: "hosting_channel",
            view: Some(123456u32),
        }
    );

    Ok(())
}
