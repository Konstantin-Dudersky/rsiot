use rsiot::components::cmp_inject_single::*;

use crate::msg::*;

pub fn cmp() -> Cmp<Msg, impl FnOnce() -> Vec<Msg>> {
    let config = Config {
        fn_output: move || vec![Msg::SingleMessage(())],
    };

    Cmp::new(config)
}
