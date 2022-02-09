mod proptest;

use super::*;

#[test]
fn test_basic() {
    assert_eq!(
        Boolean::parse(&b"#t\r\n"[..]),
        Ok((&b""[..], Boolean(true)))
    );

    assert_eq!(
        Boolean::parse(&b"#f\r\n"[..]),
        Ok((&b""[..], Boolean(false)))
    );
}
