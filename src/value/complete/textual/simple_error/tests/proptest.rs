use bytes::Bytes;
use proptest::prelude::*;

use super::SimpleError;

prop_compose! {
    fn value()(
        code in "[A-Z]+",
        msg in "[^\r\n]+"
    ) -> SimpleError {
        SimpleError::new(code, msg)
    }
}

proptest! {
    #[test]
    fn test_basic(v in value()) {
        let bytes = Bytes::try_from(v.clone()).unwrap();
        let (rest, parsed) = SimpleError::parse(&bytes).unwrap();

        assert!(rest.is_empty());
        assert_eq!(parsed, v);
    }
}
