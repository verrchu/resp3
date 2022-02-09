use bytes::Bytes;
use proptest::prelude::*;

use super::Array;

pub fn value() -> impl Strategy<Value = Array> {
    any::<Vec<()>>()
        .prop_flat_map(|ns| {
            ns.into_iter()
                .map(|_| crate::value::tests::prop::value())
                .collect::<Vec<_>>()
        })
        .prop_map(Array::from)
}

proptest! {
    #[test]
    fn test_basic(v in value()) {
        let bytes = Bytes::try_from(v.clone()).unwrap();
        let (rest, parsed) = Array::parse(&bytes).unwrap();

        assert!(rest.is_empty());
        assert_eq!(parsed, v);
    }
}
