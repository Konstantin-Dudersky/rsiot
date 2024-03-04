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
//! {"ValueInstantF64":{"value":123.0,"ts":"2000-01-01T00:00:00.000000000Z","source":"c13064d3-9460-4e82-b96c-c4d889f706c6"}}
//! ```
//!
//! В консолидолжно прочитаться это сообщение
//!
//! - через веб-интерфейс, перед запуском примера создать хеш с названием `rsiot-redis-client`,
//! задать ключ `Output`. После запуска примера в консоли должно прочитаться это сообщение
//!
//! - корректный перезапуск. При отключении Redis, или передачи неправильного сообщения в Pub/Sub

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use tracing::Level;
    use tracing_subscriber::fmt;
    use url::Url;

    use rsiot_component_core::{ComponentExecutor, ComponentExecutorConfig};
    use rsiot_extra_components::cmp_logger;
    use rsiot_messages_core::{example_message::*, *};
    use rsiot_redis_client as cmp_redis_client;

    fmt().init();

    let logger_config = cmp_logger::Config {
        level: Level::INFO,
        header: "".into(),
    };

    let redis_config = cmp_redis_client::Config {
        url: Url::parse("redis://127.0.0.1:6379")?,
        subscription_channel: ExampleMessageChannel::Output,
        fn_input: |msg: &Message<Custom>| {
            let channel = ExampleMessageChannel::Output;
            let key = msg.key.clone();
            let value = msg.serialize()?;
            Ok(Some(vec![cmp_redis_client::ConfigFnInputItem {
                channel,
                key,
                value,
            }]))
        },
        fn_output: |text: &str| {
            let msg = Message::deserialize(text)?;
            Ok(Some(vec![msg]))
        },
    };

    let executor_config = ComponentExecutorConfig {
        buffer_size: 100,
        executor_name: "redis-client-subscription".into(),
        fn_auth: |msg, _| Some(msg),
    };

    ComponentExecutor::<Custom>::new(executor_config)
        .add_cmp(cmp_logger::Cmp::new(logger_config))
        .add_cmp(cmp_redis_client::Cmp::new(redis_config))
        .wait_result()
        .await?;

    Ok(())
}

#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
fn main() {}
