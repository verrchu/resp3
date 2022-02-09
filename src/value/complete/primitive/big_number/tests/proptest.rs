use std::str::FromStr;

use bytes::Bytes;
use num_bigint::BigInt;
use proptest::prelude::*;

use super::BigNumber;

fn value() -> impl Strategy<Value = BigNumber> {
    "-?[1-9][0-9]*".prop_map(|n| BigNumber::from(BigInt::from_str(&n).unwrap()))
}

proptest! {
    #[test]
    fn test_basic(v in value()) {
        let bytes = Bytes::try_from(v.clone()).unwrap();
        let (rest, parsed) = BigNumber::parse(&bytes).unwrap();

        assert!(rest.is_empty());
        assert_eq!(parsed, v);
    }
}
