pub mod prop;

use super::*;

#[test]
fn test_basic() {
    assert_eq!(
        BlobString::parse(&b"$11\r\nhello world\r\n"[..]),
        Ok((&b""[..], BlobString(Bytes::from(b"hello world".to_vec()))))
    );
}
