use tokio::{
    main, spawn,
    time::{sleep, Duration},
};
use tracing::{info, Level};

use rsiot_component_core::ComponentCollection;
use rsiot_extra_components::{cmp_cache, cmp_inject_periodic, cmp_logger};
use rsiot_messages_core::{msg_types, ExampleMessage};

#[main]
async fn main() {
    tracing_subscriber::fmt().init();

    let cache = cmp_cache::create_cache();
    let mut counter = 0.0;

    let mut chain = ComponentCollection::new(
        100,
        vec![
            cmp_inject_periodic::new(cmp_inject_periodic::Config {
                period: Duration::from_millis(500),
                fn_periodic: move || {
                    let msg = ExampleMessage::ValueInstantF64(msg_types::Value::new(counter));
                    counter += 1.0;
                    vec![msg]
                },
            }),
            cmp_cache::new(cmp_cache::Config {
                cache: cache.clone(),
            }),
            cmp_logger::new(cmp_logger::Config {
                level: Level::INFO,
                header: "".to_string(),
            }),
        ],
    );

    let _end_task = spawn(async move {
        loop {
            {
                let lock = cache.read().await;
                info!("cache: {:?}", lock);
            }
            sleep(Duration::from_secs(5)).await;
        }
    });

    chain.spawn().await;
}
