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
        message::{example_message::*, example_service::Service},
    };
    use tracing::info;

    tracing_subscriber::fmt().init();

    let executor_config = ComponentExecutorConfig {
        buffer_size: 100,
        service: Service::example_service,
        fn_auth: |msg, _| Some(msg),
        delay_publish: Duration::from_millis(100),
    };

    let system_info_config = cmp_system_info::Config {
        period: Duration::from_secs(2),
        fn_output: |info| {
            info!("{:?}", info);
            vec![]
        },
    };

    ComponentExecutor::<Custom, Service>::new(executor_config)
        .add_cmp(cmp_system_info::Cmp::new(system_info_config))
        .wait_result()
        .await
        .unwrap();
}

#[cfg(not(feature = "cmp_system_info"))]
fn main() {
    unimplemented!()
}
