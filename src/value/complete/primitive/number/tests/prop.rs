use bytes::Bytes;
use proptest::prelude::*;

use super::Number;
use crate::value::complete::special::attribute::tests::prop::value as attr_value;

pub fn value() -> impl Strategy<Value = Number> {
    any::<i64>().prop_map(Number::from)
}

prop_compose! {
    pub fn value_with_attr()(
        val in value(),
        attr in prop::option::of(attr_value())
    ) -> Number {
        attr.map(|attr| val.clone().with_attr(attr)).unwrap_or(val)
    }
}

proptest! {
    #[test]
    fn test_basic(v in value_with_attr()) {
        let bytes = Bytes::try_from(v.clone()).unwrap();
        let (rest, parsed) = Number::parse(&bytes).unwrap();

        assert!(rest.is_empty());
        assert_eq!(parsed, v);
    }
}
