//! Пример для проверки компонента
//!
//! Проверки:
//!
//! - через веб-интрефейс, в Pub/Sub, в канал `rsiot-redis-subscriber`
//! записать сообщение {"Message0": 123}. В консоли должно прочитаться это
//! сообщение
//!
//! - через веб-интерфейс, перед запуском примера создать хеш с названием
//! `rsiot-redis-subscriber`, задать ключ `Message0`. После запуска примера
//! в консоли должно прочитаться это сообщение
//!
//! - корректный перезапуск. При отключении Redis, передачи неправильного
//! сообщения в Pub/Sub

use serde::{Deserialize, Serialize};
use tokio::main;
use tracing::Level;
use tracing_subscriber::fmt;
use url::Url;

use rsiot_component_core::ComponentChain;
use rsiot_extra_components::cmp_logger;
use rsiot_messages_core::IMessage;
use rsiot_redis_subscriber::cmp_redis_subscriber;

#[derive(Clone, Debug, Deserialize, Serialize)]
enum Messages {
    Message0(u32),
}

impl IMessage for Messages {}

#[main]
async fn main() {
    fmt().init();

    let mut chain = ComponentChain::<Messages>::new(100)
        .add_cmp(cmp_redis_subscriber::create(cmp_redis_subscriber::Config {
            url: Url::parse("redis://127.0.0.1:6379").unwrap(),
            redis_channel: "rsiot-redis-subscriber".to_string(),
        }))
        .add_cmp(cmp_logger::create(cmp_logger::Config {
            level: Level::INFO,
            header: "".into(),
        }));
    chain.spawn().await;
}
