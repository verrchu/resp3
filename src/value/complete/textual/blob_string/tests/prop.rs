use bytes::Bytes;
use proptest::prelude::*;

use super::BlobString;

fn value() -> impl Strategy<Value = BlobString> {
    any::<Vec<u8>>().prop_map(BlobString::from)
}

proptest! {
    #[test]
    fn test_basic(v in value()) {
        let bytes = Bytes::try_from(v.clone()).unwrap();
        let (rest, parsed) = BlobString::parse(&bytes).unwrap();

        assert!(rest.is_empty());
        assert_eq!(parsed, v);
    }
}
