use tokio::{main, time::Duration};

use rsiot_component_core::ComponentChain;
use rsiot_extra_components::{cmp_inject_periodic, cmp_logger, cmp_mpsc_to_mpsc};
use rsiot_messages_core::{msg_types, ExampleMessage};
use tracing::Level;

#[main]
async fn main() {
    tracing_subscriber::fmt().init();

    let mut counter = 0.0;
    let mut chain = ComponentChain::new(100)
        // Генерация сообщений
        .add_cmp(cmp_inject_periodic::new(cmp_inject_periodic::Config {
            period: Duration::from_secs(2),
            fn_periodic: move || {
                let msg = ExampleMessage::ValueInstantF64(msg_types::Value::new(counter));
                counter += 1.0;
                vec![msg]
            },
        }))
        // Пересылаем между каналами
        .add_cmp(cmp_mpsc_to_mpsc::create())
        // Логгирование
        .add_cmp(cmp_logger::create(cmp_logger::Config {
            level: Level::INFO,
            header: "".into(),
        }));
    chain.spawn().await;
}
