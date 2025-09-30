#[cfg(feature = "cmp_esp")]
mod message;

#[cfg(feature = "cmp_esp")]
#[allow(dead_code)]
#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    use std::time::Duration;

    use esp_idf_svc::sys::link_patches;
    use rsiot::{
        executor::{ComponentExecutor, ComponentExecutorConfig},
        logging::LogConfig,
    };
    use tokio::task::LocalSet;

    use message::*;
    use tracing::level_filters::LevelFilter;

    // ESP
    link_patches();

    LogConfig {
        esp_filter_level: LevelFilter::INFO,
    }
    .run()
    .unwrap();

    let executor_config = ComponentExecutorConfig {
        buffer_size: 10,
        fn_auth: |msg, _| Some(msg),
        delay_publish: Duration::from_millis(100),
        fn_tokio_metrics: |_| None,
    };

    let local_set = LocalSet::new();

    local_set.spawn_local(async {
        ComponentExecutor::<Msg>::new(executor_config)
            .wait_result()
            .await?;

        Ok(()) as anyhow::Result<()>
    });
    local_set.await;

    Ok(())
}

#[cfg(not(feature = "cmp_esp"))]
fn main() {
    unimplemented!()
}
