mod proptest;

use super::*;
use nom::error::{Error, ErrorKind};

#[test]
fn test_basic() {
    assert_eq!(
        SimpleError::parse(&b"-ERR reason\r\n"[..]),
        Ok((&b""[..], SimpleError::new("ERR", "reason")))
    );
}

#[test]
fn test_invalid_characters() {
    assert_eq!(
        SimpleError::parse(&b"-ERR some\nreason\r\n"[..]),
        Err(nom::Err::Error(Error::new(
            &b"\nreason\r\n"[..],
            ErrorKind::Tag
        )))
    );

    assert_eq!(
        SimpleError::parse(&b"-ERR some\rreason\r\n"[..]),
        Err(nom::Err::Error(Error::new(
            &b"\rreason\r\n"[..],
            ErrorKind::Tag
        )))
    );
}
