//! Запуск:
//!
//! ```rust
//! cargo run -p rsiot-extra-components --example cmp_external_fn_process_single_thread --features single-thread
//! ```

fn main() {}

// #![cfg(feature = "single-thread")]
// #![cfg(not(feature = "single-thread"))]

// use std::time::Duration;

// use futures::future::{BoxFuture, LocalBoxFuture};
// use tokio::{main, task::LocalSet, time::sleep};
// use tracing::info;

// use rsiot_component_core::{ComponentCollection, ComponentResult};
// use rsiot_extra_components::cmp_external_fn_process;
// use rsiot_messages_core::ExampleMessage;

// #[main(flavor = "current_thread")]
// async fn main() {
//     let task_set = LocalSet::new();

//     task_set
//         .spawn_local(async move {
//             let config_external_process = cmp_external_fn_process::Config {
//                 fn_process: Box::new(foo),
//             };

//             ComponentCollection::<ExampleMessage>::new(100)
//                 // .add_cmp(cmp_external_fn_process::Cmp::new(config_external_process))
//                 .wait_result()
//                 .await
//         })
//         .await;
// }

// async fn fn_process() {
//     loop {
//         info!("External fn process");
//         sleep(Duration::from_secs(2)).await;
//     }
// }

// fn foo() -> LocalBoxFuture<'static, ComponentResult> {
//     Box::pin(async { Ok(()) })
// }
