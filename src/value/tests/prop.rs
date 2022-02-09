use proptest::prelude::*;

use super::*;

pub fn value() -> impl Strategy<Value = Value> {
    prop_oneof![
        Just(Value::Null),
        complete::primitive::big_number::tests::prop::value().prop_map(Value::from),
        complete::primitive::boolean::tests::prop::value().prop_map(Value::from),
        complete::primitive::number::tests::prop::value().prop_map(Value::from),
        complete::primitive::double::tests::prop::value().prop_map(Value::from),
        complete::textual::blob_error::tests::prop::value().prop_map(Value::from),
        complete::textual::blob_string::tests::prop::value().prop_map(Value::from),
        complete::textual::simple_error::tests::prop::value().prop_map(Value::from),
        complete::textual::simple_string::tests::prop::value().prop_map(Value::from),
        complete::textual::verbatim_string::tests::prop::value().prop_map(Value::from),
        complete::recursive::array::tests::prop::value().prop_map(Value::from),
        complete::recursive::map::tests::prop::value().prop_map(Value::from),
        complete::recursive::set::tests::prop::value().prop_map(Value::from),
    ]
}
