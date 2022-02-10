pub mod prop;

use super::*;

#[test]
fn test_basic() {
    assert_eq!(
        VerbatimString::parse(&b"=15\r\ntxt:hello world\r\n"[..]),
        Ok((&b""[..], VerbatimString::txt(b"hello world".to_vec())))
    );

    assert_eq!(
        VerbatimString::parse(&b"=15\r\nmkd:hello world\r\n"[..]),
        Ok((&b""[..], VerbatimString::mkd(b"hello world".to_vec())))
    );
}
