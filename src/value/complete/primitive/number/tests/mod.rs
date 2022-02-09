pub mod prop;

use super::*;

#[test]
fn test_positive_number() {
    assert_eq!(
        Number::parse(&b":1234\r\n"[..]),
        Ok((&b""[..], Number(1234)))
    );
}

#[test]
fn test_negative_number() {
    assert_eq!(
        Number::parse(&b":-1234\r\n"[..]),
        Ok((&b""[..], Number(-1234)))
    );
}
