//! Tags for TMI messages

use std::collections::HashMap;

/// A type alias for a HashMap whose keys are [`&str`] and values [`TagValue`].
/// Uses slice [`&str`] instead of owned [`String`] in order to avoid data duplication.
pub type Tags<'a> = HashMap<&'a str, TagValue<'a>>;

/// Possible values of message tags.
#[derive(Debug, PartialEq)]
pub enum TagValue<'a> {
    /// Represents a parsed sequence of numbers of type u32.
    Number(u32),
    /// Represents a parsed sequence of numbers of type u64.
    Timestamp(u64),
    /// Boolean values represents literal "1" (true) or "0" (false).
    ///
    /// Note that a single digit number "1" or "0" may be represented
    /// as a Boolean value instead of a Number value.
    /// Type conversion should therefore be done by the user code.
    Boolean(bool),
    /// Strings represent an unparsed string literal.
    String(&'a str),
    /// None represents literal empty string "".
    None,
}

impl<'a> TagValue<'a> {
    /// Returns a TagValue variant based on the given [`&str`].
    pub fn new(val: &'a str) -> TagValue<'a> {
        match val {
            "" => TagValue::None,
            "0" => TagValue::Boolean(false),
            "1" => TagValue::Boolean(true),
            _ => {
                if let Ok(num) = val.parse::<u32>() {
                    TagValue::Number(num)
                } else if let Ok(tm) = val.parse::<u64>() {
                    TagValue::Timestamp(tm)
                } else {
                    TagValue::String(val)
                }
            }
        }
    }
}
