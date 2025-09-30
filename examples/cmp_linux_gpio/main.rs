#[cfg(feature = "cmp_linux_gpio")]
mod config_inject_periodic;
#[cfg(feature = "cmp_linux_gpio")]
mod config_linux_gpio;
#[cfg(feature = "cmp_linux_gpio")]
mod config_logger;
#[cfg(feature = "cmp_linux_gpio")]
mod message;

#[cfg(feature = "cmp_linux_gpio")]
#[tokio::main]
async fn main() {
    use std::time::Duration;

    use rsiot::executor::{ComponentExecutor, ComponentExecutorConfig};
    use tracing::Level;

    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let config_executor = ComponentExecutorConfig {
        buffer_size: 1000,
        fn_auth: |msg, _| Some(msg),
        delay_publish: Duration::from_millis(1000),
        fn_tokio_metrics: |_| None,
    };

    ComponentExecutor::new(config_executor)
        .add_cmp(config_inject_periodic::cmp())
        .add_cmp(config_linux_gpio::cmp())
        .add_cmp(config_logger::cmp())
        .wait_result()
        .await
        .unwrap();
}

#[cfg(not(feature = "cmp_linux_gpio"))]
fn main() {
    unimplemented!()
}
