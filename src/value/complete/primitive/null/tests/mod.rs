pub mod prop;

use super::*;

#[test]
fn test_basic() {
    assert_eq!(Null::parse(&b"_\r\n"[..]), Ok((&b""[..], Null::default())));
}

#[test]
fn test_codec() {
    assert_eq!(
        Null::parse(&Bytes::try_from(Null::default()).unwrap()),
        Ok((&b""[..], Null::default()))
    );
}
