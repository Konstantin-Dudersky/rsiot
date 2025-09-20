#[cfg(feature = "cmp_svg")]
mod config_external_process;
#[cfg(feature = "cmp_svg")]
mod config_inject_periodic;
#[cfg(feature = "cmp_svg")]
mod config_svg;
#[cfg(feature = "cmp_svg")]
mod message;

#[cfg(feature = "cmp_svg")]
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
        .add_cmp(config_svg::cmp())
        .add_cmp(config_external_process::cmp())
        .add_cmp(config_inject_periodic::cmp())
        .wait_result()
        .await
        .unwrap();
}

#[cfg(not(feature = "cmp_svg"))]
fn main() {
    unimplemented!()
}
