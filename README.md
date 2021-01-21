# tmi-parser

[![cratesio-badge](https://img.shields.io/crates/v/tmi-parser)](https://crates.io/crates/tmi-parser)

## Description

A simple, dependency-less library for parsing IRC-based TMI message.

This library is made with simplicity and performance in mind using only Rust's pattern matching.

Still a __WIP__.

## Known issues

* If fed with incorrect input, `Message::parse` may panic.

* `Message` tags are not checked.

* Single digit number `0` or `1` will be interpreted as Boolean `false` or `true`.

## License

Licensed under the terms of the MIT license.
