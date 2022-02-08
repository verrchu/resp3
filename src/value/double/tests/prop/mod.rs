use proptest::prelude::*;

use super::*;

mod parts;
mod sign;

pub fn value() -> impl Strategy<Value = Double> {
    prop_oneof![
        sign::value().prop_map(Double::Inf),
        parts::value().prop_map(|parts| Double::from_parts(parts).unwrap())
    ]
}

proptest! {
    #[test]
    fn test_basic(v in value()) {
        let bytes = Bytes::try_from(v.clone()).unwrap();
        let (rest, parsed) = Double::parse(&bytes).unwrap();

        assert!(rest.is_empty());
        assert_eq!(parsed, v);
    }
}
