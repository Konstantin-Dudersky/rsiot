//! Запуск:
//!
//! ```rust
//! cargo run -p rsiot-extra-components --example cmp_external_fn_process --features single-thread
//!
//! cargo run -p rsiot-extra-components --example cmp_external_fn_process
//! ```

// #![cfg(feature = "single-thread")]
// #![cfg(not(feature = "single-thread"))]

use std::time::Duration;

use futures::future::{BoxFuture, LocalBoxFuture};
use tokio::{main, task::LocalSet, time::sleep};
use tracing::{info, level_filters::LevelFilter};

use rsiot_component_core::{ComponentExecutor, ComponentResult};
use rsiot_extra_components::cmp_external_fn_process;
use rsiot_messages_core::{example_message::*, *};

#[main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();

    let config_external_process = cmp_external_fn_process::Config {
        // fn_process: Box::new(foo),
        fn_process: Box::new(multi_thread),
    };

    let task_set = LocalSet::new();
    task_set.spawn_local(async move {
        ComponentExecutor::<Custom>::new(100, "cmp_external_fn_process_single_thread")
            .add_cmp(cmp_external_fn_process::Cmp::new(config_external_process))
            .wait_result()
            .await
    });
    task_set.await;
}

async fn fn_process() -> ComponentResult {
    loop {
        info!("External fn process");
        sleep(Duration::from_secs(2)).await;
    }
}

fn single_thread() -> LocalBoxFuture<'static, ComponentResult> {
    Box::pin(async { fn_process().await })
}

fn multi_thread() -> BoxFuture<'static, ComponentResult> {
    Box::pin(async { fn_process().await })
}
