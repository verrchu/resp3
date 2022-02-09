use bytes::Bytes;
use proptest::prelude::*;

use super::Array;

pub fn value() -> impl Strategy<Value = Array> {
    prop::collection::vec(crate::value::tests::prop::value(), 0..=10).prop_map(Array::from)
}

proptest! {
    #[test]
    fn test_basic(v in value()) {
        let bytes = Bytes::try_from(v.clone()).unwrap();
        let (rest, parsed) = Array::parse(&bytes).unwrap();

        assert!(rest.is_empty());
        assert_eq!(parsed, v);
    }
}
