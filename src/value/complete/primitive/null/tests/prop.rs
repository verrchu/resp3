use bytes::Bytes;
use proptest::prelude::*;

use super::Null;
use crate::value::complete::recursive::attribute::tests::prop::value as attr_value;

pub fn value() -> impl Strategy<Value = Null> {
    Just(Null::default())
}

pub fn value_with_attr() -> impl Strategy<Value = Null> {
    prop::option::of(attr_value()).prop_map(|attr| Null { attr })
}

proptest! {
    #[test]
    fn test_basic(v in value_with_attr()) {
        let bytes = Bytes::try_from(v.clone()).unwrap();
        let (rest, parsed) = Null::parse(&bytes).unwrap();

        assert!(rest.is_empty());
        assert_eq!(parsed, v);
    }
}
