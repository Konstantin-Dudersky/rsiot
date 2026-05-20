use rsiot::components::cmp_os_process::*;
use tracing::info;

use crate::msg::*;

pub fn cmp_chronyc() -> Cmp<Msg> {
    let config = Config {
        commands: vec![ConfigCommand {
            fn_input: |msg| {
                let cmds = match msg {
                    Msg::Init(_) => {
                        vec![format!(
                            "chronyc add server {} iburst maxpoll 3",
                            "192.168.123.99"
                        )]
                    }
                    Msg::PeriodicSyncTime(_) => vec!["chronyc -a makestep".to_string()],
                };

                Some(cmds)
            },
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
