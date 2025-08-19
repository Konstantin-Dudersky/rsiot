//! Запуск:
//!
//! ```bash
//! cargo run --example cmp_filesystem --features="cmp_filesystem, serde-toml"
//! cargo run --example cmp_filesystem --features="cmp_filesystem, serde-json"
//! ```

#[cfg(feature = "cmp_filesystem")]
mod config_filesystem;
#[cfg(feature = "cmp_filesystem")]
mod config_inject_periodic;
#[cfg(feature = "cmp_filesystem")]
mod config_logger;
#[cfg(feature = "cmp_filesystem")]
mod messages;

#[cfg(feature = "cmp_filesystem")]
#[tokio::main]
async fn main() {
    use std::time::Duration;

    use rsiot::executor::*;

    tracing_subscriber::fmt().init();

    // executor ------------------------------------------------------------------------------------
    let config_executor = ComponentExecutorConfig {
        buffer_size: 100,
        fn_auth: |msg, _| Some(msg),
        delay_publish: Duration::from_millis(100),
        fn_tokio_metrics: |_| None,
    };
    ComponentExecutor::new(config_executor)
        .add_cmp(config_inject_periodic::new())
        .add_cmp(config_filesystem::new())
        .add_cmp(config_logger::new())
        .wait_result()
        .await
        .unwrap();
}

#[cfg(not(feature = "cmp_filesystem"))]
fn main() {
    unimplemented!()
}
