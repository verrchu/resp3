use proptest::prelude::*;

use super::Map;

pub fn value() -> impl Strategy<Value = Map> {
    any::<Vec<()>>()
        .prop_flat_map(|ns| {
            ns.into_iter()
                .map(|_| {
                    (
                        crate::value::tests::prop::value(),
                        crate::value::tests::prop::value(),
                    )
                })
                .collect::<Vec<_>>()
        })
        .prop_map(Map::from)
}
