use bytes::Bytes;
use proptest::prelude::*;

use super::SimpleString;

fn value() -> impl Strategy<Value = SimpleString> {
    "[^\r\n]+".prop_map(SimpleString::from)
}

proptest! {
    #[test]
    fn test_basic(v in value()) {
        let bytes = Bytes::try_from(v.clone()).unwrap();
        let (rest, parsed) = SimpleString::parse(&bytes).unwrap();

        assert!(rest.is_empty());
        assert_eq!(parsed, v);
    }
}
