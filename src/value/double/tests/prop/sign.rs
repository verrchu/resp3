use proptest::prelude::*;

use super::*;

pub fn value() -> impl Strategy<Value = Sign> {
    prop_oneof![Just(Sign::Plus), Just(Sign::Minus)]
}
