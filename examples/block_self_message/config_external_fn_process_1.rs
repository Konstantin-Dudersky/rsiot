use std::time::Duration;

use rsiot::components::cmp_external_fn_process::*;
use rsiot::executor::{CmpInOut, CmpResult};
use rsiot::message::MsgDataBound;
use tokio::task::JoinSet;
use tokio::time::sleep;
use tracing::error;

use crate::messages::*;

pub fn cmp() -> rsiot::executor::Component<Config<Msg>, Msg> {
    let config = Config {
        fn_process: Box::new(fn_process_wrapper),
    };

    Cmp::new(config)
}

#[cfg(feature = "single-thread")]
fn fn_process_wrapper(
    msg_bus: CmpInOut<Msg>,
) -> futures::future::LocalBoxFuture<'static, CmpResult> {
    Box::pin(async { fn_process(msg_bus).await })
}

#[cfg(not(feature = "single-thread"))]
fn fn_process_wrapper(msg_bus: CmpInOut<Msg>) -> futures::future::BoxFuture<'static, CmpResult> {
    Box::pin(async { fn_process(msg_bus).await })
}

async fn fn_process(mut msg_bus: CmpInOut<Msg>) -> CmpResult {
    let mut task_set = JoinSet::new();

    let msg_bus_clone = msg_bus.clone();
    task_set.spawn(async move {
        loop {
            let msg = Msg::GenerateMessage(());
            msg_bus_clone.send_output(msg.to_message()).await.unwrap();
            sleep(Duration::from_millis(1_000)).await
        }
    });

    task_set.spawn(async move {
        while let Ok(msg) = msg_bus.recv_input().await {
            let Some(msg) = msg.get_custom_data() else {
                continue;
            };
            match msg {
                Msg::GenerateMessage(_) => error!("Received GenerateMessage in 1"),
            }
        }
    });

    task_set.join_all().await;

    Ok(())
}
