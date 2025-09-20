use std::fs::write;

use rsiot::{
    components::cmp_external_fn_process::*,
    executor::{CmpInOut, CmpResult, Component},
};

use crate::message::*;

pub fn cmp() -> Component<Config<Msg>, Msg> {
    let config = Config {
        fn_process: Box::new(fn_process_wrapper),
    };

    Cmp::new(config)
}

async fn fn_process(mut input: CmpInOut<Msg>) -> CmpResult {
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
fn fn_process_wrapper(input: CmpInOut<Msg>) -> BoxFuture<'static, CmpResult> {
    Box::pin(async { fn_process(input).await })
}

#[cfg(feature = "single-thread")]
use futures::future::LocalBoxFuture;

#[cfg(feature = "single-thread")]
fn fn_process_wrapper(input: CmpInOut<Msg>) -> LocalBoxFuture<'static, CmpResult> {
    Box::pin(async { fn_process(input).await })
}
