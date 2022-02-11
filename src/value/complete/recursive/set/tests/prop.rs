use bytes::Bytes;
use proptest::prelude::*;

use super::Set;
use crate::value::complete::recursive::attribute::tests::prop::value as attr_value;

pub fn value() -> impl Strategy<Value = Set> {
    prop::collection::vec(crate::value::tests::prop::value(), 0..=10).prop_map(Set::from)
}

prop_compose! {
    pub fn value_with_attr()(
        val in value(),
        attr in prop::option::of(attr_value())
    ) -> Set {
        attr.map(|attr| val.clone().with_attr(attr)).unwrap_or(val)
    }
}

proptest! {
    #[test]
    fn test_basic(v in value_with_attr()) {
        let bytes = Bytes::try_from(v.clone()).unwrap();
        let (rest, parsed) = Set::parse(&bytes).unwrap();

        assert!(rest.is_empty());
        assert_eq!(parsed, v);
    }
}
