use std::time::Duration;

use rsiot::{
    components::cmp_external_fn_process::*,
    executor::{CmpResult, MsgBusInput, MsgBusOutput},
    message::{MsgData, system_messages::System},
};
use tokio::time::sleep;
use tracing::{error, info, warn};

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
    drop(output);
    while let Ok(msg) = input.recv_input().await {
        match msg.data {
            MsgData::System(msg) => match msg {
                System::Lagged => warn!("Lagged message"),
                _ => continue,
            },
            MsgData::Custom(msg) => match msg {
                Msg::Counter(v) => info!("Counter: {v}"),
            },
        }

        sleep(Duration::from_millis(110)).await
    }

    error!("Component stopped");

    Ok(())
}
