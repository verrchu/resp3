use bytes::Bytes;
use proptest::prelude::*;

use super::BlobError;

prop_compose! {
    pub fn value()(
        code in "[A-Z]+",
        msg in any::<Vec<u8>>()
    ) -> BlobError {
        BlobError::new(code, msg)
    }
}

proptest! {
    #[test]
    fn test_basic(v in value()) {
        let bytes = Bytes::try_from(v.clone()).unwrap();
        let (rest, parsed) = BlobError::parse(&bytes).unwrap();

        assert!(rest.is_empty());
        assert_eq!(parsed, v);
    }
}
