use proptest::prelude::*;

use super::Set;

pub fn value() -> impl Strategy<Value = Set> {
    any::<Vec<()>>()
        .prop_flat_map(|ns| {
            ns.into_iter()
                .map(|_| crate::value::tests::prop::value())
                .collect::<Vec<_>>()
        })
        .prop_map(Set::from)
}
