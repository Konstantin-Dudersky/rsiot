use std::time::Duration;

use crate::{
    executor::{MsgBusLinker, sleep},
    message::MsgDataBound,
};

use super::{Config, Error};

pub async fn fn_process<TMsg>(
    config: Config<TMsg>,
    msgbus_linker: MsgBusLinker<TMsg>,
) -> super::Result<()>
where
    TMsg: MsgDataBound,
{
    let msgs = (config.fn_output)();
    let output = msgbus_linker.output();
    msgbus_linker.close();
    for msg in msgs {
        let msg = msg.to_message();
        output
            .send(msg)
            .await
            .map_err(|_| Error::TokioSyncMpscSend)?;
    }
    drop(output);
    loop {
        sleep(Duration::MAX).await;
    }
}
