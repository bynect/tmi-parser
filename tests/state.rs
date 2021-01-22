use std::io::Result;
use tmi_parser::*;

#[test]
fn parse_roomstate() -> Result<()> {
    let room1 = "@emote-only=<emote-only>;followers-only=<followers-only>;r9k=<r9k>;slow=<slow>;\
                subs-only=<subs-only> :tmi.twitch.tv ROOMSTATE #<channel>\r\n";
    let room2 =
        "@emote-only=0;followers-only=0;r9k=0;slow=0;subs-only=0 :tmi.twitch.tv ROOMSTATE #dallas";
    let room3 = ":tmi.twitch.tv ROOMSTATE #<channel>";

    let mut tags1 = Tags::default();
    tags1.insert("emote-only", TagValue::String("<emote-only>"));
    tags1.insert("followers-only", TagValue::String("<followers-only>"));
    tags1.insert("r9k", TagValue::String("<r9k>"));
    tags1.insert("slow", TagValue::String("<slow>"));
    tags1.insert("subs-only", TagValue::String("<subs-only>"));

    assert_eq!(
        Message::parse(room1)?,
        Message::Roomstate {
            tags: Some(tags1),
            chan: "<channel>",
        }
    );

    let mut tags2 = Tags::default();
    tags2.insert("emote-only", TagValue::Boolean(false));
    tags2.insert("followers-only", TagValue::Boolean(false));
    tags2.insert("r9k", TagValue::Boolean(false));
    tags2.insert("slow", TagValue::Boolean(false));
    tags2.insert("subs-only", TagValue::Boolean(false));

    assert_eq!(
        Message::parse(room2)?,
        Message::Roomstate {
            tags: Some(tags2),
            chan: "dallas",
        }
    );

    assert_eq!(
        Message::parse(room3)?,
        Message::Roomstate {
            tags: None,
            chan: "<channel>",
        }
    );

    Ok(())
}

#[test]
fn parse_userstate() -> Result<()> {
    let user1 = "@badge-info=<badge-info>;badges=<badges>;color=<color>;display-name=<display-name>;\
                emote-sets=<emote-sets>;mod=<mod>;subscriber=<subscriber>;turbo=<turbo>;user-type=<user-type> \
                :tmi.twitch.tv USERSTATE #<channel>\r\n";
    let user2 = "@badge-info=;badges=staff/1;color=#0D4200;display-name=ronni;emote-sets=0,33,50,237,793,2126,3517,4578,5569,9400,10337,12239;\
                mod=1;subscriber=1;turbo=1;user-type=staff :tmi.twitch.tv USERSTATE #dallas";
    let user3 = ":tmi.twitch.tv USERSTATE #<channel>";

    let mut tags1 = Tags::default();
    tags1.insert("badge-info", TagValue::String("<badge-info>"));
    tags1.insert("badges", TagValue::String("<badges>"));
    tags1.insert("color", TagValue::String("<color>"));
    tags1.insert("display-name", TagValue::String("<display-name>"));
    tags1.insert("emote-sets", TagValue::String("<emote-sets>"));
    tags1.insert("mod", TagValue::String("<mod>"));
    tags1.insert("subscriber", TagValue::String("<subscriber>"));
    tags1.insert("turbo", TagValue::String("<turbo>"));
    tags1.insert("user-type", TagValue::String("<user-type>"));

    assert_eq!(
        Message::parse(user1)?,
        Message::Userstate {
            tags: Some(tags1),
            chan: "<channel>",
        }
    );

    let mut tags2 = Tags::default();
    tags2.insert("badge-info", TagValue::None);
    tags2.insert("badges", TagValue::String("staff/1"));
    tags2.insert(
        "color",
        TagValue::Color(u32::from_str_radix("0D4200", 16).unwrap()),
    );
    tags2.insert("display-name", TagValue::String("ronni"));
    tags2.insert(
        "emote-sets",
        TagValue::String("0,33,50,237,793,2126,3517,4578,5569,9400,10337,12239"),
    );
    tags2.insert("mod", TagValue::Boolean(true));
    tags2.insert("subscriber", TagValue::Boolean(true));
    tags2.insert("turbo", TagValue::Boolean(true));
    tags2.insert("user-type", TagValue::String("staff"));

    assert_eq!(
        Message::parse(user2)?,
        Message::Userstate {
            tags: Some(tags2),
            chan: "dallas",
        }
    );

    assert_eq!(
        Message::parse(user3)?,
        Message::Userstate {
            tags: None,
            chan: "<channel>",
        }
    );

    Ok(())
}
