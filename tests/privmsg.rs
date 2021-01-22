use std::io::Result;
use tmi_parser::*;

#[test]
fn parse_privmsg() -> Result<()> {
    let msg1 = "PRIVMSG #<channel> :This is a sample message\r\n";
    let msg2 = ":<user>!<user>@<user>.tmi.twitch.tv PRIVMSG #<channel> :This is a sample message";
    let msg3 = "@badge-info=;badges=global_mod/1,turbo/1;color=#0D4200;display-name=ronni;\
                emotes=25:0-4,12-16/1902:6-10;id=b34ccfc7-4977-403a-8a94-33c6bac34fb8;mod=0;\
                room-id=1337;subscriber=0;tmi-sent-ts=1507246572675;turbo=1;user-id=1337;\
                user-type=global_mod :ronni!ronni@ronni.tmi.twitch.tv PRIVMSG #ronni :Kappa Keepo Kappa";

    assert_eq!(
        Message::parse(msg1)?,
        Message::Privmsg {
            tags: None,
            chan: "<channel>",
            msg: "This is a sample message",
        }
    );

    assert_eq!(
        Message::parse(msg2)?,
        Message::Privmsg {
            tags: None,
            chan: "<channel>",
            msg: "This is a sample message",
        }
    );

    let mut tags = Tags::default();
    tags.insert("badge-info", TagValue::None);
    tags.insert("badges", TagValue::String("global_mod/1,turbo/1"));
    tags.insert(
        "color",
        TagValue::Color(u32::from_str_radix("0D4200", 16).unwrap()),
    );
    tags.insert("display-name", TagValue::String("ronni"));
    tags.insert("emotes", TagValue::String("25:0-4,12-16/1902:6-10"));
    tags.insert(
        "id",
        TagValue::String("b34ccfc7-4977-403a-8a94-33c6bac34fb8"),
    );
    tags.insert("mod", TagValue::Boolean(false));
    tags.insert("room-id", TagValue::Number(1337u32));
    tags.insert("subscriber", TagValue::Boolean(false));
    tags.insert("tmi-sent-ts", TagValue::Timestamp(1507246572675u64));
    tags.insert("turbo", TagValue::Boolean(true));
    tags.insert("user-id", TagValue::Number(1337u32));
    tags.insert("user-type", TagValue::String("global_mod"));

    assert_eq!(
        Message::parse(msg3)?,
        Message::Privmsg {
            tags: Some(tags),
            chan: "ronni",
            msg: "Kappa Keepo Kappa",
        }
    );

    Ok(())
}
