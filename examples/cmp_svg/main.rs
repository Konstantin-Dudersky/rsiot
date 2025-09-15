mod config_external_process;
mod config_inject_periodic;
mod config_svg;
mod message;

use std::time::Duration;

use rsiot::executor::{ComponentExecutor, ComponentExecutorConfig};
use tokio::main;
use tracing::{Level, info};

#[main]
async fn main() {
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
