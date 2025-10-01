// #[cfg(feature = "cmp_os_process")]
// mod config_logger;
#[cfg(feature = "cmp_os_process")]
mod config_os_process;
#[cfg(feature = "cmp_os_process")]
mod message;

#[cfg(feature = "cmp_os_process")]
#[tokio::main]
async fn main() {
    use std::time::Duration;

    use rsiot::executor::{ComponentExecutor, ComponentExecutorConfig};

    tracing_subscriber::fmt().init();

    let config_executor = ComponentExecutorConfig {
        buffer_size: 100,
        fn_auth: |msg, _| Some(msg),
        delay_publish: Duration::from_millis(100),
        fn_tokio_metrics: |_| None,
    };

    ComponentExecutor::new(config_executor)
        .add_cmp(config_os_process::cmp())
        // .add_cmp(config_logger::cmp())
        .wait_result()
        .await
        .unwrap();
}

#[cfg(not(feature = "cmp_os_process"))]
fn main() {}
