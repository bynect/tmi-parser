use std::io::Result;
use tmi_parser::*;

#[test]
fn tagvalue() -> Result<()> {
    let tag1 = "staff/1,bits/1000";
    let tag2 = "#0D4200";
    let tag3 = "ronni";
    let tag4 = "25:0-4,12-16/1902:6-10";
    let tag5 = "b34ccfc7-4977-403a-8a94-33c6bac34fb8";
    let tag6 = "0";

    assert_eq!(format!("{}", TagValue::new(tag1)), tag1);
    assert_eq!(format!("{}", TagValue::new(tag2)), tag2);
    assert_eq!(format!("{}", TagValue::new(tag3)), tag3);
    assert_eq!(format!("{}", TagValue::new(tag4)), tag4);
    assert_eq!(format!("{}", TagValue::new(tag5)), tag5);
    assert_eq!(format!("{}", TagValue::new(tag6)), tag6);

    Ok(())
}
