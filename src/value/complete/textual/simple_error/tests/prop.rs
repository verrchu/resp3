use bytes::Bytes;
use proptest::prelude::*;

use super::SimpleError;
use crate::value::complete::special::attribute::tests::prop::value as attr_value;

prop_compose! {
    pub fn value()(
        code in "[A-Z]+",
        msg in "[^\r\n]+"
    ) -> SimpleError {
        SimpleError::new(code, msg)
    }
}

prop_compose! {
    pub fn value_with_attr()(
        val in value(),
        attr in prop::option::of(attr_value())
    ) -> SimpleError {
        attr.map(|attr| val.clone().with_attr(attr)).unwrap_or(val)
    }
}

proptest! {
    #[test]
    fn test_basic(v in value_with_attr()) {
        let bytes = Bytes::try_from(v.clone()).unwrap();
        let (rest, parsed) = SimpleError::parse(&bytes).unwrap();

        assert!(rest.is_empty());
        assert_eq!(parsed, v);
    }
}
