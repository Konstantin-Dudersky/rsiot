use std::fs::write;

use rsiot::{
    components::cmp_external_fn_process::*,
    executor::{CmpResult, Component, MsgBusInput, MsgBusOutput},
};

use crate::message::*;

pub fn cmp() -> Component<Config<Msg>, Msg> {
    let config = Config {
        fn_process: Box::new(fn_process_wrapper),
    };

    Cmp::new(config)
}

async fn fn_process(mut input: MsgBusInput<Msg>, output: MsgBusOutput<Msg>) -> CmpResult {
    drop(output);
    while let Ok(msg) = input.recv_input().await {
        let Some(msg) = msg.get_custom_data() else {
            continue;
        };

        #[allow(clippy::single_match)]
        match msg {
            Msg::OutputSvg(svg) => {
                write("./examples/cmp_svg/files/output.svg", &svg).unwrap();
            }
            _ => (),
        }
    }

    Ok(())
}

#[cfg(not(feature = "single-thread"))]
use futures::future::BoxFuture;

#[cfg(not(feature = "single-thread"))]
fn fn_process_wrapper(
    input: MsgBusInput<Msg>,
    output: MsgBusOutput<Msg>,
) -> BoxFuture<'static, CmpResult> {
    Box::pin(async { fn_process(input, output).await })
}

#[cfg(feature = "single-thread")]
use futures::future::LocalBoxFuture;

#[cfg(feature = "single-thread")]
fn fn_process_wrapper(
    input: MsgBusInput<Msg>,
    output: MsgBusOutput<Msg>,
) -> LocalBoxFuture<'static, CmpResult> {
    Box::pin(async { fn_process(input, output).await })
}
