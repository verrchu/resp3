pub mod prop;

use super::*;

#[test]
fn test_basic() {
    assert_eq!(
        BlobError::parse(&b"!10\r\nERR reason\r\n"[..]),
        Ok((&b""[..], BlobError::new("ERR", b"reason".to_vec())))
    );
}
