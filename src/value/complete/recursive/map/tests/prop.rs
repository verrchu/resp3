use bytes::Bytes;
use proptest::prelude::*;

use super::Map;
use crate::value::complete::recursive::attribute::tests::prop::value as attr_value;

pub fn value() -> impl Strategy<Value = Map> {
    prop::collection::vec(
        (
            crate::value::tests::prop::value(),
            crate::value::tests::prop::value(),
        ),
        0..=10,
    )
    .prop_map(Map::from)
}

prop_compose! {
    pub fn value_with_attr()(
        val in value(),
        attr in prop::option::of(attr_value())
    ) -> Map {
        attr.map(|attr| val.clone().with_attr(attr)).unwrap_or(val)
    }
}

proptest! {
    #[test]
    fn test_basic(v in value_with_attr()) {
        let bytes = Bytes::try_from(v.clone()).unwrap();
        let (rest, parsed) = Map::parse(&bytes).unwrap();

        assert!(rest.is_empty());
        assert_eq!(parsed, v);
    }
}
