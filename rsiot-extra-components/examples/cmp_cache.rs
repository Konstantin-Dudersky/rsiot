use serde::{Deserialize, Serialize};
use tokio::{
    main, spawn,
    time::{sleep, Duration},
};
use tracing::{info, Level};

use rsiot_component_core::ComponentChain;
use rsiot_extra_components::{cmp_cache, cmp_inject_periodic, cmp_logger};
use rsiot_messages_core::IMessage;

#[derive(Clone, Debug, Deserialize, Serialize)]
enum Message {
    Message0(f64),
    Message1(f64),
    CombineMessage(f64, f64),
}

impl IMessage for Message {}

#[main]
async fn main() {
    tracing_subscriber::fmt().init();

    let cache = cmp_cache::create_cache();
    let mut counter = 0.0;

    let mut chain = ComponentChain::init(100)
        .start_cmp(cmp_inject_periodic::new(cmp_inject_periodic::Config {
            period: Duration::from_millis(500),
            fn_periodic: move || {
                let msg = Message::Message0(counter);
                counter += 1.0;
                vec![msg]
            },
        }))
        .then_cmp(cmp_cache::new(cmp_cache::Config {
            cache: cache.clone(),
        }))
        .end_cmp(cmp_logger::create(cmp_logger::Config {
            level: Level::INFO,
        }));

    let _end_task = spawn(async move {
        loop {
            {
                let lock = cache.lock().await;
                info!("cache: {:?}", lock);
            }
            sleep(Duration::from_secs(5)).await;
        }
    });

    chain.spawn().await;
}
