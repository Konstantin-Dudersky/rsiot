//! Запуск:
//!
//! ```rust
//! cargo run -p rsiot-extra-components --example cmp_external_fn_process --features single-thread
//!
//! cargo run -p rsiot-extra-components --example cmp_external_fn_process
//! ```

#[cfg(feature = "executor")]
#[tokio::main(flavor = "current_thread")]
async fn main() {
    use std::time::Duration;

    #[cfg(not(feature = "single-thread"))]
    use futures::future::BoxFuture;
    #[cfg(feature = "single-thread")]
    use futures::future::LocalBoxFuture;
    use tokio::{task::LocalSet, time::sleep};
    use tracing::{info, level_filters::LevelFilter};

    use rsiot::{
        components::cmp_external_fn_process,
        executor::{CmpInOut, CmpResult, ComponentExecutor, ComponentExecutorConfig},
        message::{example_message::*, example_service::*, *},
    };

    async fn fn_process<TMsg>(_input: CmpInOut<TMsg>) -> CmpResult
    where
        TMsg: MsgDataBound,
    {
        loop {
            info!("External fn process");
            sleep(Duration::from_secs(2)).await;
        }
    }

    #[cfg(feature = "single-thread")]
    fn fn_process_wrapper<TMsg>(input: CmpInOut<TMsg>) -> LocalBoxFuture<'static, CmpResult>
    where
        TMsg: MsgDataBound + 'static,
    {
        Box::pin(async { fn_process(input).await })
    }

    #[cfg(not(feature = "single-thread"))]
    fn fn_process_wrapper<TMsg>(input: CmpInOut<TMsg>) -> BoxFuture<'static, CmpResult>
    where
        TMsg: MsgDataBound + 'static,
    {
        Box::pin(async { fn_process(input).await })
    }

    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();

    let config_external_process = cmp_external_fn_process::Config {
        fn_process: Box::new(fn_process_wrapper),
    };

    let executor_config = ComponentExecutorConfig {
        buffer_size: 100,
        service: Service::example_service,
        fn_auth: |msg, _| Some(msg),
        delay_publish: Duration::from_secs(0),
    };

    let task_set = LocalSet::new();
    task_set.spawn_local(async move {
        ComponentExecutor::<Custom>::new(executor_config)
            .add_cmp(cmp_external_fn_process::Cmp::new(config_external_process))
            .wait_result()
            .await
    });
    task_set.await;
}

#[cfg(not(feature = "executor"))]
fn main() {}
