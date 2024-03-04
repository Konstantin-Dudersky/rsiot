//! Запуск:
//! ```bash
//! cargo run -p rsiot-extra-components --example cmp_inject_periodic
//!
//! cargo run -p rsiot-extra-components --example cmp_inject_periodic --features single-thread
//! ```

use tokio::{main, task::LocalSet, time::Duration};

use rsiot_component_core::{ComponentExecutor, ComponentExecutorConfig};
use rsiot_extra_components::{cmp_inject_periodic, cmp_logger};
use rsiot_messages_core::{example_message::*, Message};
use tracing::{level_filters::LevelFilter, Level};

#[main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();

    let executor_config = ComponentExecutorConfig {
        buffer_size: 100,
        executor_name: "cmp_inject_periodic".into(),
        fn_auth: |msg, _| Some(msg),
    };

    let logger_config = cmp_logger::Config {
        level: Level::INFO,
        header: "Logger: ".into(),
    };

    let mut counter = 0.0;
    let inject_config = cmp_inject_periodic::Config {
        period: Duration::from_secs(2),
        fn_periodic: move || {
            let msg = Custom::ValueInstantF64(counter);
            let msg = Message::new_custom(msg);
            counter += 1.0;
            vec![msg]
        },
    };

    let local_set = LocalSet::new();

    local_set.spawn_local(async {
        ComponentExecutor::<Custom>::new(executor_config)
            .add_cmp(cmp_logger::Cmp::new(logger_config))
            .add_cmp(cmp_inject_periodic::Cmp::new(inject_config))
            .wait_result()
            .await
            .unwrap();
    });

    local_set.await;

    Ok(())
}
