use proptest::prelude::{prop as p, *};

use super::*;

pub fn value() -> impl Strategy<Value = Value> {
    let strat = prop_oneof![
        complete::primitive::null::tests::prop::value().prop_map(Value::from),
        complete::primitive::big_number::tests::prop::value().prop_map(Value::from),
        complete::primitive::boolean::tests::prop::value().prop_map(Value::from),
        complete::primitive::number::tests::prop::value().prop_map(Value::from),
        complete::primitive::double::tests::prop::value().prop_map(Value::from),
        complete::textual::blob_error::tests::prop::value().prop_map(Value::from),
        complete::textual::blob_string::tests::prop::value().prop_map(Value::from),
        complete::textual::simple_error::tests::prop::value().prop_map(Value::from),
        complete::textual::simple_string::tests::prop::value().prop_map(Value::from),
        complete::textual::verbatim_string::tests::prop::value().prop_map(Value::from),
    ];

    strat.prop_recursive(2, 64, 16, |e| {
        prop_oneof![
            p::collection::vec(e.clone(), 0..16)
                .prop_map(|values| Value::from(Array::from(values))),
            p::collection::vec(e.clone(), 0..16).prop_map(|values| Value::from(Set::from(values))),
            p::collection::vec((e.clone(), e.clone()), 0..16)
                .prop_map(|values| Value::from(Map::from(values))),
        ]
    })
}

proptest! {
    #[test]
    fn test_basic(v in value()) {
        let bytes = Bytes::try_from(v.clone()).unwrap();
        let (rest, parsed) = Value::parse(&bytes).unwrap();

        assert!(rest.is_empty());
        assert_eq!(parsed, v);
    }
}
