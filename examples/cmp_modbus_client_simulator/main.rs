//! Простейший пример использования клиента Modbus
//!
//! Для тестирования можно использовать образ docker oitc/modbus-server
//!
//! Выполняется две операции:
//! - раз в 2 секунды на сервер в регистр 0 записывается значение счетчика (`input_config`)
//! - раз в 2 секунды считывается значение регистра 0 (`periodic_config`) и отправляется в логгер
//!

#[cfg(feature = "cmp_modbus_client")]
mod config_inject_periodic;
#[cfg(feature = "cmp_modbus_client")]
mod config_logger;
#[cfg(feature = "cmp_modbus_client")]
mod config_modbus_client;
#[cfg(feature = "cmp_modbus_client")]
mod message;

#[cfg(feature = "cmp_modbus_client")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use tokio::time::Duration;
    use tracing_subscriber::fmt;

    use rsiot::executor::{ComponentExecutor, ComponentExecutorConfig};

    // логгирование
    fmt().init();

    let executor_config = ComponentExecutorConfig {
        buffer_size: 100,
        fn_auth: |msg, _| Some(msg),
        delay_publish: Duration::from_millis(100),
        fn_tokio_metrics: |_| None,
    };

    ComponentExecutor::new(executor_config)
        .add_cmp(config_inject_periodic::cmp())
        .add_cmp(config_modbus_client::cmp())
        .add_cmp(config_logger::cmp())
        .wait_result()
        .await?;
    Ok(())
}

#[cfg(not(feature = "cmp_modbus_client"))]
fn main() {}
