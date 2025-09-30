use std::time::Duration;

use rsiot::{
    components::cmp_external_fn_process::*,
    executor::{CmpResult, MsgBusInput, MsgBusOutput},
    message::MsgDataBound,
};
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
    input: MsgBusInput<Msg>,
    output: MsgBusOutput<Msg>,
) -> futures::future::LocalBoxFuture<'static, CmpResult> {
    Box::pin(async { fn_process(input, output).await })
}

#[cfg(not(feature = "single-thread"))]
fn fn_process_wrapper(
    input: MsgBusInput<Msg>,
    output: MsgBusOutput<Msg>,
) -> futures::future::BoxFuture<'static, CmpResult> {
    Box::pin(async { fn_process(input, output).await })
}

async fn fn_process(mut input: MsgBusInput<Msg>, output: MsgBusOutput<Msg>) -> CmpResult {
    let mut task_set = JoinSet::new();

    task_set.spawn(async move {
        loop {
            let msg = Msg::GenerateMessage(());
            output.send(msg.to_message()).await.unwrap();
            sleep(Duration::from_millis(1_000)).await
        }
    });

    task_set.spawn(async move {
        while let Ok(msg) = input.recv().await {
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
