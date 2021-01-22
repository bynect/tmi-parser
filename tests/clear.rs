use std::io::Result;
use tmi_parser::*;

#[test]
fn parse_clearchat() -> Result<()> {
    let chat1 = ":tmi.twitch.tv CLEARCHAT #dallas";
    let chat2 = "\t:tmi.twitch.tv CLEARCHAT #dallas :ronni";
    let chat3 = "@ban-duration=<ban-duration> :tmi.twitch.tv CLEARCHAT #<channel> :<user>";

    assert_eq!(
        Message::parse(chat1)?,
        Message::Clearchat {
            tags: None,
            chan: "dallas",
            usr: None,
        }
    );

    assert_eq!(
        Message::parse(chat2)?,
        Message::Clearchat {
            tags: None,
            chan: "dallas",
            usr: Some("ronni"),
        }
    );

    let mut tags = Tags::default();
    tags.insert("ban-duration", TagValue::String("<ban-duration>"));

    assert_eq!(
        Message::parse(chat3)?,
        Message::Clearchat {
            tags: Some(tags),
            chan: "<channel>",
            usr: Some("<user>"),
        }
    );

    Ok(())
}

#[test]
fn parse_clearmsg() -> Result<()> {
    let msg1 = "  @login=<login>;target-msg-id=<target-msg-id> :tmi.twitch.tv CLEARMSG #<channel> :<message>";
    let msg2 =
        "   @login=ronni;target-msg-id=abc-123-def :tmi.twitch.tv CLEARMSG #dallas :HeyGuys  \r\n";

    let mut tags1 = Tags::default();
    tags1.insert("login", TagValue::String("<login>"));
    tags1.insert("target-msg-id", TagValue::String("<target-msg-id>"));

    assert_eq!(
        Message::parse(msg1)?,
        Message::Clearmsg {
            tags: Some(tags1),
            chan: "<channel>",
            msg: "<message>",
        }
    );

    let mut tags2 = Tags::default();
    tags2.insert("login", TagValue::String("ronni"));
    tags2.insert("target-msg-id", TagValue::String("abc-123-def"));

    assert_eq!(
        Message::parse(msg2)?,
        Message::Clearmsg {
            tags: Some(tags2),
            chan: "dallas",
            msg: "HeyGuys",
        }
    );

    Ok(())
}
