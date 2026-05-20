mod cmp_chronyc;

pub use cmp_chronyc::cmp_chronyc;

use rsiot::{components::cmp_os_process::*, executor::Component};
use tracing::info;

use crate::msg::*;

pub fn cmp() -> Component<Config<Msg>, Msg> {
    let config = Config {
        commands: vec![ConfigCommand {
            fn_input: |_msg| None,
            fn_output: |results| {
                for result in results {
                    info!("Result: {}", result.stdout);
                }
                None
            },
        }],
    };

    Cmp::new(config)
}
