use std::io::Result;
use tmi_parser::*;

#[test]
fn parse_join() -> Result<()> {
    let join1 = "JOIN #<channel>";
    let join2 = ":ronni!ronni@ronni.tmi.twitch.tv JOIN #dallas";

    assert_eq!(Message::parse(join1)?, Message::Join { chan: "<channel>" });

    assert_eq!(Message::parse(join2)?, Message::Join { chan: "dallas" });

    Ok(())
}
