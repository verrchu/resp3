use bytes::Bytes;
use proptest::prelude::*;

use super::SimpleError;

prop_compose! {
    fn value()(
        code in "[A-Z]{1,8}",
        msg in "[^\r\n]{1,10}"
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
