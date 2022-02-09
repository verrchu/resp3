use proptest::prelude::*;

use super::*;

prop_compose! {
    pub fn value()(
        sign in sign::value(),
        int in any::<u64>(),
        frac in any::<Option<u64>>()
    ) -> Parts {
        Parts { sign, int, frac }
    }
}
