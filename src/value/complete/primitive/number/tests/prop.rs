use bytes::Bytes;
use proptest::prelude::*;

use super::Number;

pub fn value() -> impl Strategy<Value = Number> {
    any::<i64>().prop_map(Number::from)
}

proptest! {
    #[test]
    fn test_basic(v in value()) {
        let bytes = Bytes::try_from(v.clone()).unwrap();
        let (rest, parsed) = Number::parse(&bytes).unwrap();

        assert!(rest.is_empty());
        assert_eq!(parsed, v);
    }
}
