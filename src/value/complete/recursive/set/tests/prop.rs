use bytes::Bytes;
use proptest::prelude::*;

use super::Set;

pub fn value() -> impl Strategy<Value = Set> {
    prop::collection::vec(crate::value::tests::prop::value(), 0..=10).prop_map(Set::from)
}

proptest! {
    #[test]
    fn test_basic(v in value()) {
        let bytes = Bytes::try_from(v.clone()).unwrap();
        let (rest, parsed) = Set::parse(&bytes).unwrap();

        assert!(rest.is_empty());
        assert_eq!(parsed, v);
    }
}
