//! Tags for TMI messages

use std::{collections::HashMap};
use std::hash::BuildHasherDefault;

/// [`Tags`] is type alias for a [`HashMap`] whose keys are [`&str`] and values [`TagValue`].
/// Uses slice [`&str`] instead of owned [`String`] in order to avoid data duplication.
/// Also uses a custom hasher which implements the 'Fnv 1a' hash function.
///
/// # Examples
///
/// ```
/// # use tmi_parser::*;
/// let mut map = Tags::default();
/// map.insert("hello", TagValue::String("world"));
/// # assert_eq!(*map.get("hello").unwrap(), TagValue::String("world"));
/// ````
pub type Tags<'a> = HashMap<&'a str, TagValue<'a>, BuildHasherDefault<hash::TagsHasher>>;

/// [`TagsHasher`] is an internal implementation of the 'Fnv 1a' hash function.
/// It lives under the private module hash and was made only for improving [`Tags`] performance.
/// Since [`Tags`] will contain few items in average, the faster the hash, the better the performance,
mod hash {
    use std::hash::Hasher;

    pub struct TagsHasher(u64);

    impl Default for TagsHasher {
        #[inline]
        fn default() -> TagsHasher {
            TagsHasher(14695981039346656037)
        }
    }

    impl Hasher for TagsHasher {
        #[inline]
        fn finish(&self) -> u64 {
            self.0
        }

        #[inline]
        fn write(&mut self, bytes: &[u8]) {
            let TagsHasher(mut hash) = *self;

            for byte in bytes.iter() {
                hash ^= *byte as u64;
                hash = hash.wrapping_mul(1099511628211);
            }

            *self = TagsHasher(hash);
        }
    }
}

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
                } else if val.starts_with('#') {
                    // Try to convert hexadecimal values, used by the 'color' tag, to Number.
                    if let Ok(num) = u32::from_str_radix(&val[1..], 16) {
                        TagValue::Number(num)
                    } else {
                        TagValue::String(val)
                    }
                } else {
                    TagValue::String(val)
                }
            }
        }
    }
}
