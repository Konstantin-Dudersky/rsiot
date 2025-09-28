use rsiot::{
    components::cmp_external_fn_process::*,
    executor::{CmpResult, MsgBusInput, MsgBusOutput},
};
use tracing::info;

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

async fn fn_process(mut input: MsgBusInput<Msg>, _output: MsgBusOutput<Msg>) -> CmpResult {
    while let Ok(msg) = input.recv_input().await {
        let Some(msg) = msg.get_custom_data() else {
            continue;
        };
        match msg {
            Msg::CounterInhectPeriodic(v) => info!("Counter inject: {v}"),
        }
    }

    Ok(())
}
