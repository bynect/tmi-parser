use std::io::Result;
use tmi_parser::*;

#[test]
fn parse_part() -> Result<()> {
    let part1 = "PART #<channel>";
    let part2 = ":ronni!ronni@ronni.tmi.twitch.tv PART #dallas";

    assert_eq!(Message::parse(part1)?, Message::Part { chan: "<channel>" });

    assert_eq!(Message::parse(part2)?, Message::Part { chan: "dallas" });

    Ok(())
}
