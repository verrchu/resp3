pub mod prop;

use super::*;
use nom::error::{Error, ErrorKind};

#[test]
fn test_basic() {
    assert_eq!(
        SimpleString::parse(&b"+hello world\r\n"[..]),
        Ok((&b""[..], SimpleString(String::from("hello world"))))
    );
}

#[test]
fn test_invalid_characters() {
    assert_eq!(
        SimpleString::parse(&b"+hello\nworld\r\n"[..]),
        Err(nom::Err::Error(Error::new(
            &b"\nworld\r\n"[..],
            ErrorKind::Tag
        )))
    );

    assert_eq!(
        SimpleString::parse(&b"+hello\rworld\r\n"[..]),
        Err(nom::Err::Error(Error::new(
            &b"\rworld\r\n"[..],
            ErrorKind::Tag
        )))
    );
}
