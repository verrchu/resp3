use std::str::FromStr;

use bytes::Bytes;
use num_bigint::BigInt;
use proptest::prelude::*;

use super::BigNumber;
use crate::value::complete::recursive::attribute::tests::prop::value as attr_value;

pub fn value() -> impl Strategy<Value = BigNumber> {
    "-?[1-9][0-9]*".prop_map(|n| BigNumber::from(BigInt::from_str(&n).unwrap()))
}

prop_compose! {
    pub fn value_with_attr()(
        val in value(),
        attr in prop::option::of(attr_value())
    ) -> BigNumber {
        attr.map(|attr| val.clone().with_attr(attr)).unwrap_or(val)
    }
}

proptest! {
    #[test]
    fn test_basic(v in value_with_attr()) {
        let bytes = Bytes::try_from(v.clone()).unwrap();
        let (rest, parsed) = BigNumber::parse(&bytes).unwrap();

        assert!(rest.is_empty());
        assert_eq!(parsed, v);
    }
}
