use std::time::Duration;

use crate::executor::{ComponentExecutor, ComponentExecutorConfig};
use tokio::task::LocalSet;

use super::messages::*;

#[allow(dead_code)]
#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    // executor ------------------------------------------------------------------------------------

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
