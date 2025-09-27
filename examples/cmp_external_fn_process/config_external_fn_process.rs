use rsiot::components::cmp_external_fn_process::*;
use rsiot::executor::{CmpInOut, CmpResult};
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
    msg_bus: CmpInOut<Msg>,
) -> futures::future::LocalBoxFuture<'static, CmpResult> {
    Box::pin(async { fn_process(msg_bus).await })
}

#[cfg(not(feature = "single-thread"))]
fn fn_process_wrapper(msg_bus: CmpInOut<Msg>) -> futures::future::BoxFuture<'static, CmpResult> {
    Box::pin(async { fn_process(msg_bus).await })
}

async fn fn_process(mut msg_bus: CmpInOut<Msg>) -> CmpResult {
    while let Ok(msg) = msg_bus.recv_input().await {
        let Some(msg) = msg.get_custom_data() else {
            continue;
        };
        match msg {
            Msg::CounterInhectPeriodic(v) => info!("Counter inject: {v}"),
        }
    }

    Ok(())
}
