//! Запуск:
//!
//! ```bash
//!
//! ```

#[cfg(feature = "cmp_filesystem")]
mod config_file_appender;
#[cfg(feature = "cmp_filesystem")]
mod config_inject_periodic;
#[cfg(feature = "cmp_filesystem")]
mod messages;

#[cfg(feature = "cmp_filesystem")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use std::time::Duration;

    use rsiot::executor::*;

    tracing_subscriber::fmt().init();

    // executor ------------------------------------------------------------------------------------
    let config_executor = ComponentExecutorConfig {
        buffer_size: 10,
        fn_auth: |msg, _| Some(msg),
        delay_publish: Duration::from_millis(2),
        fn_tokio_metrics: |_| None,
    };
    ComponentExecutor::new(config_executor)
        .add_cmp(config_inject_periodic::cmp())
        .add_cmp(config_file_appender::cmp())
        .wait_result()
        .await?;

    Err(anyhow::Error::msg("Program end"))
}

#[cfg(not(feature = "cmp_filesystem"))]
fn main() {
    unimplemented!()
}
