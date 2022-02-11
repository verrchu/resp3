use bytes::Bytes;
use proptest::prelude::*;

use super::BlobString;
use crate::value::complete::special::attribute::tests::prop::value as attr_value;

pub fn value() -> impl Strategy<Value = BlobString> {
    any::<Vec<u8>>().prop_map(BlobString::from)
}

prop_compose! {
    pub fn value_with_attr()(
        val in value(),
        attr in prop::option::of(attr_value())
    ) -> BlobString {
        attr.map(|attr| val.clone().with_attr(attr)).unwrap_or(val)
    }
}

proptest! {
    #[test]
    fn test_basic(v in value_with_attr()) {
        let bytes = Bytes::try_from(v.clone()).unwrap();
        let (rest, parsed) = BlobString::parse(&bytes).unwrap();

        assert!(rest.is_empty());
        assert_eq!(parsed, v);
    }
}
