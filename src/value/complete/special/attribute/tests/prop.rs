use bytes::Bytes;
use proptest::prelude::*;

use super::Attribute;

pub fn value() -> impl Strategy<Value = Attribute> {
    prop::collection::vec(
        (
            crate::value::tests::prop::value(),
            crate::value::tests::prop::value(),
        ),
        0..=10,
    )
    .prop_map(Attribute::from)
}

proptest! {
    #[test]
    fn test_basic(v in value()) {
        let bytes = Bytes::try_from(v.clone()).unwrap();
        let (rest, parsed) = Attribute::parse(&bytes).unwrap();

        assert!(rest.is_empty());
        assert_eq!(parsed, v);
    }
}
