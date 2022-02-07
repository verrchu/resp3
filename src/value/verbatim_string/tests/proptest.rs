use bytes::Bytes;
use proptest::prelude::*;

use super::VerbatimString;

fn value() -> impl Strategy<Value = VerbatimString> {
    prop_oneof![
        any::<Vec<u8>>().prop_map(VerbatimString::txt),
        any::<Vec<u8>>().prop_map(VerbatimString::mkd),
    ]
}

proptest! {
    #[test]
    fn test_basic(v in value()) {
        let bytes = Bytes::try_from(v.clone()).unwrap();
        let (rest, parsed) = VerbatimString::parse(&bytes).unwrap();

        assert!(rest.is_empty());
        assert_eq!(parsed, v);
    }
}
