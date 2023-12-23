use tokio::{main, time::Duration};

use rsiot_component_core::ComponentChain;
use rsiot_extra_components::{cmp_inject_periodic, cmp_logger};
use rsiot_messages_core::{msg_types, ExampleMessage};
use tracing::{level_filters::LevelFilter, Level};

#[main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();

    let mut counter = 0.0;
    let mut chain = ComponentChain::<ExampleMessage>::new(
        100,
        vec![
            cmp_inject_periodic::new(cmp_inject_periodic::Config {
                period: Duration::from_secs(2),
                fn_periodic: move || {
                    let msg = ExampleMessage::ValueInstantF64(msg_types::Value::new(counter));
                    counter += 1.0;
                    vec![msg]
                },
            }),
            cmp_logger::new(cmp_logger::Config {
                level: Level::INFO,
                header: "Logger: ".into(),
            }),
        ],
    );
    chain.spawn().await;
}
