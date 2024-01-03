//! Пример для проверки компонента
//!
//! Запуск:
//!
//! ```bash
//! cargo run -p rsiot-redis-client --example subscription
//! ```
//!
//! Проверки:
//!
//! - через веб-интрефейс, в Pub/Sub, в канал `Output` записать сообщение
//!
//! ```json
//! {"ValueInstantF64":{"value":123.0,"ts":"2000-01-01T00:00:00.000000000Z"}}
//! ```
//!
//! В консолидолжно прочитаться это сообщение
//!
//! - через веб-интерфейс, перед запуском примера создать хеш с названием `rsiot-redis-client`,
//! задать ключ `Output`. После запуска примера в консоли должно прочитаться это сообщение
//!
//! - корректный перезапуск. При отключении Redis, или передачи неправильного сообщения в Pub/Sub

use tokio::main;
use tracing::Level;
use tracing_subscriber::fmt;
use url::Url;

use rsiot_component_core::ComponentCollection;
use rsiot_extra_components::cmp_logger;
use rsiot_messages_core::{ExampleMessage, ExampleMessageChannel};
use rsiot_redis_client::cmp_redis_client;

#[main]
async fn main() {
    fmt().init();

    let mut chain = ComponentCollection::<ExampleMessage>::new(
        100,
        vec![
            cmp_redis_client::new(cmp_redis_client::Config {
                url: Url::parse("redis://127.0.0.1:6379").unwrap(),
                subscription_channel: ExampleMessageChannel::Output,
                fn_input: |_| vec![ExampleMessageChannel::Output],
            }),
            cmp_logger::new(cmp_logger::Config {
                level: Level::INFO,
                header: "".into(),
            }),
        ],
    );
    chain.spawn().await.unwrap();
}
