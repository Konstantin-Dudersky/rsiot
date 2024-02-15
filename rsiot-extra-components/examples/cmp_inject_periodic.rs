use tokio::{main, task::LocalSet, time::Duration};

use rsiot_component_core::ComponentExecutor;
use rsiot_extra_components::{cmp_inject_periodic, cmp_logger};
use rsiot_messages_core::{ExampleMessage, MsgContent};
use tracing::{level_filters::LevelFilter, Level};

#[main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();

    let local_set = LocalSet::new();

    local_set.spawn_local(async {
        let logger_config = cmp_logger::Config {
            level: Level::INFO,
            header: "Logger: ".into(),
        };

        let mut counter = 0.0;
        let inject_config = cmp_inject_periodic::Config {
            period: Duration::from_secs(2),
            fn_periodic: move || {
                let msg = ExampleMessage::ValueInstantF64(MsgContent::new(counter));
                counter += 1.0;
                vec![msg]
            },
        };

        ComponentExecutor::<ExampleMessage>::new(100, "cmp_inject_periodic")
            .add_cmp(cmp_logger::Cmp::new(logger_config))
            .add_cmp(cmp_inject_periodic::Cmp::new(inject_config))
            .wait_result()
            .await
            .unwrap();
    });

    local_set.await;

    Ok(())
}
