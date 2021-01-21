use std::io::Result;
use tmi_parser::*;

#[test]
fn parse_ping() -> Result<()> {
    let ping1 = "PING :tmi.twitch.tv";
    let ping2 = "      PING :tmi.twitch.tv \r\n";

    assert_eq!(Message::parse(ping1)?, Message::Ping);

    assert_eq!(Message::parse(ping2)?, Message::Ping);

    Ok(())
}

#[test]
fn parse_pong() -> Result<()> {
    let pong1 = " PONG :tmi.twitch.tv";
    let pong2 = "PONG :tmi.twitch.tv \r\n";

    assert_eq!(Message::parse(pong1)?, Message::Pong);

    assert_eq!(Message::parse(pong2)?, Message::Pong);

    Ok(())
}

#[test]
fn parse_reconnect() -> Result<()> {
    let rec1 = "RECONNECT";
    let rec2 = "  RECONNECT \r\n";

    assert_eq!(Message::parse(rec1)?, Message::Reconnect);

    assert_eq!(Message::parse(rec2)?, Message::Reconnect);

    Ok(())
}
