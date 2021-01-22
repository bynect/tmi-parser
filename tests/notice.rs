use std::io::Result;
use tmi_parser::*;

#[test]
fn parse_notice() -> Result<()> {
    let note1 = "@msg-id=<msg id> :tmi.twitch.tv NOTICE #<channel> :<message>";
    let note2 =
        "@msg-id=slow_off :tmi.twitch.tv NOTICE #dallas :This room is no longer in slow mode.\r\n";

    let mut tags1 = Tags::new();
    tags1.insert("msg-id", TagValue::String("<msg id>"));

    assert_eq!(
        Message::parse(note1)?,
        Message::Notice {
            tags: Some(tags1),
            chan: "<channel>",
            msg: "<message>",
        }
    );

    let mut tags2 = Tags::new();
    tags2.insert("msg-id", TagValue::String("slow_off"));

    assert_eq!(
        Message::parse(note2)?,
        Message::Notice {
            tags: Some(tags2),
            chan: "dallas",
            msg: "This room is no longer in slow mode.",
        }
    );

    Ok(())
}

#[test]
fn parse_usernotice() -> Result<()> {
    let note1 = "@badge-info=<badge-info>;badges=<badges>;color=<color>;display-name=<display-name>;\
                emotes=<emotes>;id=<id-of-msg>;login=<user>;mod=<mod>;msg-id=<msg-id>;room-id=<room-id>;\
                subscriber=<subscriber>;system-msg=<system-msg>;tmi-sent-ts=<timestamp>;turbo=<turbo>;\
                user-id=<user-id>;user-type=<user-type> :tmi.twitch.tv USERNOTICE #<channel> :<message>";

    let note2 = "@badge-info=;badges=staff/1,broadcaster/1,turbo/1;color=#008000;display-name=ronni;emotes=;\
                id=db25007f-7a18-43eb-9379-80131e44d633;login=ronni;mod=0;msg-id=resub;msg-param-cumulative-months=6;\
                msg-param-streak-months=2;msg-param-should-share-streak=1;msg-param-sub-plan=Prime;msg-param-sub-plan-name=Prime;\
                room-id=1337;subscriber=1;system-msg=ronni\\shas\\ssubscribed\\sfor\\s6\\smonths!;tmi-sent-ts=1507246572675;\
                turbo=1;user-id=1337;user-type=staff :tmi.twitch.tv USERNOTICE #dallas :Great stream -- keep it up!\r\n";

    let mut tags1 = Tags::new();
    tags1.insert("badge-info", TagValue::String("<badge-info>"));
    tags1.insert("badges", TagValue::String("<badges>"));
    tags1.insert("color", TagValue::String("<color>"));
    tags1.insert("display-name", TagValue::String("<display-name>"));
    tags1.insert("emotes", TagValue::String("<emotes>"));
    tags1.insert("id", TagValue::String("<id-of-msg>"));
    tags1.insert("login", TagValue::String("<user>"));
    tags1.insert("mod", TagValue::String("<mod>"));
    tags1.insert("msg-id", TagValue::String("<msg-id>"));
    tags1.insert("room-id", TagValue::String("<room-id>"));
    tags1.insert("subscriber", TagValue::String("<subscriber>"));
    tags1.insert("system-msg", TagValue::String("<system-msg>"));
    tags1.insert("tmi-sent-ts", TagValue::String("<timestamp>"));
    tags1.insert("turbo", TagValue::String("<turbo>"));
    tags1.insert("user-id", TagValue::String("<user-id>"));
    tags1.insert("user-type", TagValue::String("<user-type>"));

    assert_eq!(
        Message::parse(note1)?,
        Message::Usernotice {
            tags: Some(tags1),
            chan: "<channel>",
            msg: "<message>",
        }
    );

    let mut tags2 = Tags::new();
    tags2.insert("badge-info", TagValue::None);
    tags2.insert("badges", TagValue::String("staff/1,broadcaster/1,turbo/1"));
    tags2.insert(
        "color",
        TagValue::Number(u32::from_str_radix("008000", 16).unwrap()),
    );
    tags2.insert("display-name", TagValue::String("ronni"));
    tags2.insert("emotes", TagValue::None);
    tags2.insert(
        "id",
        TagValue::String("db25007f-7a18-43eb-9379-80131e44d633"),
    );
    tags2.insert("login", TagValue::String("ronni"));
    tags2.insert("mod", TagValue::Boolean(false));
    tags2.insert("msg-id", TagValue::String("resub"));
    tags2.insert("msg-param-cumulative-months", TagValue::Number(6));
    tags2.insert("msg-param-streak-months", TagValue::Number(2));
    tags2.insert("msg-param-should-share-streak", TagValue::Boolean(true));
    tags2.insert("msg-param-sub-plan", TagValue::String("Prime"));
    tags2.insert("msg-param-sub-plan-name", TagValue::String("Prime"));
    tags2.insert("room-id", TagValue::Number(1337u32));
    tags2.insert("subscriber", TagValue::Boolean(true));
    tags2.insert(
        "system-msg",
        TagValue::String("ronni\\shas\\ssubscribed\\sfor\\s6\\smonths!"),
    );
    tags2.insert("tmi-sent-ts", TagValue::Timestamp(1507246572675u64));
    tags2.insert("turbo", TagValue::Boolean(true));
    tags2.insert("user-id", TagValue::Number(1337u32));
    tags2.insert("user-type", TagValue::String("staff"));

    assert_eq!(
        Message::parse(note2)?,
        Message::Usernotice {
            tags: Some(tags2),
            chan: "dallas",
            msg: "Great stream -- keep it up!",
        }
    );

    Ok(())
}
