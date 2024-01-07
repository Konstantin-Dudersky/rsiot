use tokio::{main, task::LocalSet, time::Duration};

use rsiot_component_core2::ComponentCollection;
use rsiot_extra_components::{cmp_inject_periodic2, cmp_logger2};
use rsiot_messages_core::{msg_types, ExampleMessage};
use tracing::{level_filters::LevelFilter, Level};

#[main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();

    let local_set = LocalSet::new();

    local_set.spawn_local(async {
        let logger_config = cmp_logger2::Config {
            level: Level::INFO,
            header: "Logger: ".into(),
        };

        let mut counter = 0.0;
        let inject_config = cmp_inject_periodic2::Config {
            period: Duration::from_secs(2),
            fn_periodic: move || {
                let msg = ExampleMessage::ValueInstantF64(msg_types::Value::new(counter));
                counter += 1.0;
                vec![msg]
            },
        };

        ComponentCollection::<ExampleMessage>::new(100)
            .add_cmp(cmp_logger2::Cmp::new(logger_config))
            .add_cmp(cmp_inject_periodic2::Cmp::new(inject_config))
            .wait_result()
            .await
            .unwrap();
    });

    local_set.await;

    Ok(())
}
