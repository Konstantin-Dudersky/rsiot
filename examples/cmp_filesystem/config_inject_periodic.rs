use std::time::Duration;

use rsiot::components::cmp_inject_periodic;

use super::messages::*;

pub fn new(
) -> rsiot::executor::Component<cmp_inject_periodic::Config<Msg, impl FnMut() -> Vec<Msg>>, Msg> {
    let config = cmp_inject_periodic::Config {
        period: Duration::from_secs(2),
        fn_periodic: move || {
            let msg = Msg::InjPeriodic(InjPeriodic::Increase);
            vec![msg]
        },
    };

    cmp_inject_periodic::Cmp::new(config)
}
