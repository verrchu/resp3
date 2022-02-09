use bytes::Bytes;
use proptest::prelude::*;

use super::Boolean;

pub fn value() -> impl Strategy<Value = Boolean> {
    any::<bool>().prop_map(Boolean::from)
}

proptest! {
    #[test]
    fn test_basic(v in value()) {
        let bytes = Bytes::try_from(v.clone()).unwrap();
        let (rest, parsed) = Boolean::parse(&bytes).unwrap();

        assert!(rest.is_empty());
        assert_eq!(parsed, v);
    }
}
