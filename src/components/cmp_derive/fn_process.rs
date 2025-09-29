use tokio::task::JoinSet;

use crate::executor::{CmpInOut, MsgBusInput, MsgBusOutput, join_set_spawn};
use crate::message::*;

use super::{Config, DeriveItemProcess, Error};

pub async fn fn_process<TMsg>(
    msgbus_linker: CmpInOut<TMsg>,
    config: Config<TMsg>,
) -> super::Result<()>
where
    TMsg: MsgDataBound + 'static,
{
    let mut task_set = JoinSet::new();

    for item in config.derive_items {
        join_set_spawn(
            &mut task_set,
            "cmp_derive",
            task_process_derive_item(msgbus_linker.input(), msgbus_linker.output(), item),
        );
    }

    drop(msgbus_linker);

    while let Some(res) = task_set.join_next().await {
        res??
    }
    Ok(())
}

async fn task_process_derive_item<TMsg>(
    mut input: MsgBusInput<TMsg>,
    output: MsgBusOutput<TMsg>,
    mut derive_item: Box<dyn DeriveItemProcess<TMsg>>,
) -> super::Result<()>
where
    TMsg: MsgDataBound,
{
    while let Ok(msg) = input.recv().await {
        let Some(msg) = msg.get_custom_data() else {
            continue;
        };
        let msgs = derive_item.process(&msg);
        let Some(msgs) = msgs else { continue };
        for msg in msgs {
            let msg = Message::new_custom(msg);
            output
                .send(msg)
                .await
                .map_err(|e| Error::TokioSynBroadcast(e.to_string()))?
        }
    }
    Ok(())
}
