//! Запуск
//!
//! ```bash
//! cargo run --example cmp_system_info --features "cmp_system_info" --target="x86_64-unknown-linux-gnu"
//! ```

#[cfg(feature = "cmp_system_info")]
#[tokio::main()]
async fn main() {
    use std::time::Duration;

    use rsiot::{
        components::cmp_system_info,
        executor::{ComponentExecutor, ComponentExecutorConfig},
        message::{example_message::*, *},
    };

    let executor_config = ComponentExecutorConfig {
        buffer_size: 100,
        executor_name: "cmp_system_info".into(),
        fn_auth: |msg, _| Some(msg),
    };

    let system_info_config = cmp_system_info::Config {
        period: Duration::from_secs(2),
        fn_output: || vec![],
    };

    ComponentExecutor::<Custom>::new(executor_config)
        .add_cmp(cmp_system_info::Cmp::new(system_info_config))
        .wait_result()
        .await
        .unwrap();
}

#[cfg(not(feature = "cmp_system_info"))]
fn main() {
    unimplemented!()
}
