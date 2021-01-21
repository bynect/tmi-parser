use std::io::Result;
use tmi_parser::*;

#[test]
fn parse_pass() -> Result<()> {
    let pass1 = "PASS oauth:mypassworddhasdsa";
    let pass2 = "PASS oauth:hello\r\n";

    assert_eq!(
        Message::parse(pass1)?,
        Message::Pass {
            pass: "oauth:mypassworddhasdsa"
        }
    );

    assert_eq!(
        Message::parse(pass2)?,
        Message::Pass {
            pass: "oauth:hello"
        }
    );

    Ok(())
}
