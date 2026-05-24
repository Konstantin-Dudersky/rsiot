#[cfg(feature = "cmp_plc")]
mod cfg_inject_periodic;
#[cfg(feature = "cmp_plc")]
mod cfg_plc;
#[cfg(feature = "cmp_plc")]
mod msg;

#[cfg(feature = "cmp_plc")]
#[tokio::main(flavor = "current_thread")]
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
        .add_cmp(cfg_plc::cmp())
        .add_cmp(cfg_inject_periodic::cmp())
        .wait_result()
        .await
        .unwrap();
}

#[cfg(not(feature = "cmp_plc"))]
fn main() {}
