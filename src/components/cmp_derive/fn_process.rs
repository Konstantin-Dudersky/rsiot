use tokio::sync::mpsc;
use tokio::task::JoinSet;

use crate::executor::{MsgBusLinker, join_set_spawn};
use crate::message::*;

use super::{
    BufferBound, COMPONENT_NAME, Config, ConfigOutputSend, Error, task_input::Input,
    task_output::Output, task_period::Period,
};

pub async fn fn_process<TMsg, TBuffer>(
    msgbus_linker: MsgBusLinker<TMsg>,
    config: Config<TMsg, TBuffer>,
) -> Result<(), Error>
where
    TMsg: MsgDataBound + 'static,
    TBuffer: 'static + BufferBound,
{
    let (ch_tx_buffer, ch_rx_buffer) = mpsc::channel(5);

    let mut task_set = JoinSet::new();

    let task = Input {
        input: msgbus_linker.input(),
        output: ch_tx_buffer.clone(),
        fn_input: config.fn_input,
    };
    join_set_spawn(
        &mut task_set,
        format!("{COMPONENT_NAME} | input"),
        task.spawn(),
    );

    if let ConfigOutputSend::Periodic(period) = config.output_send {
        let task = Period {
            output: ch_tx_buffer,
            period,
        };
        join_set_spawn(
            &mut task_set,
            format!("{COMPONENT_NAME} | Period"),
            task.spawn(),
        );
    }

    let task = Output {
        input: ch_rx_buffer,
        output: msgbus_linker.output(),
        fn_output: config.fn_output,
        output_send: config.output_send,
    };
    join_set_spawn(
        &mut task_set,
        format!("{COMPONENT_NAME} | output"),
        task.spawn(),
    );

    msgbus_linker.close();

    while let Some(res) = task_set.join_next().await {
        res??
    }

    Ok(())
}
