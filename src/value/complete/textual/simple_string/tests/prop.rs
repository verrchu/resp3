use bytes::Bytes;
use proptest::prelude::*;

use super::SimpleString;
use crate::value::complete::recursive::attribute::tests::prop::value as attr_value;

pub fn value() -> impl Strategy<Value = SimpleString> {
    "[^\r\n]+".prop_map(SimpleString::from)
}

prop_compose! {
    pub fn value_with_attr()(
        val in value(),
        attr in prop::option::of(attr_value())
    ) -> SimpleString {
        attr.map(|attr| val.clone().with_attr(attr)).unwrap_or(val)
    }
}

proptest! {
    #[test]
    fn test_basic(v in value_with_attr()) {
        let bytes = Bytes::try_from(v.clone()).unwrap();
        let (rest, parsed) = SimpleString::parse(&bytes).unwrap();

        assert!(rest.is_empty());
        assert_eq!(parsed, v);
    }
}
