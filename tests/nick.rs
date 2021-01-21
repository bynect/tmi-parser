use std::io::Result;
use tmi_parser::*;

#[test]
fn parse_nick() -> Result<()> {
    let nick1 = "NICK justinfan232456";
    let nick2 = "NICK justinfan2adasdasd\r\n";

    assert_eq!(
        Message::parse(nick1)?,
        Message::Nick {
            nick: "justinfan232456"
        }
    );

    assert_eq!(
        Message::parse(nick2)?,
        Message::Nick {
            nick: "justinfan2adasdasd"
        }
    );

    Ok(())
}
