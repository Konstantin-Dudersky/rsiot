use tokio::task::JoinSet;

use rsiot_component_core::{CmpInput, CmpOutput};
use rsiot_messages_core::message_v2::MsgDataBound;

use super::{Config, DeriveItemProcess, Error};

pub async fn fn_process<TMsg>(
    input: CmpInput<TMsg>,
    output: CmpOutput<TMsg>,
    config: Config<TMsg>,
) -> super::Result<()>
where
    TMsg: MsgDataBound + 'static,
{
    let mut task_set = JoinSet::new();

    for item in config.derive_items {
        task_set.spawn(task_process_derive_item(
            input.clone(),
            output.clone(),
            item,
        ));
    }

    while let Some(res) = task_set.join_next().await {
        res??
    }
    Ok(())
}

async fn task_process_derive_item<TMsg>(
    mut input: CmpInput<TMsg>,
    output: CmpOutput<TMsg>,
    mut derive_item: Box<dyn DeriveItemProcess<TMsg>>,
) -> super::Result<()>
where
    TMsg: MsgDataBound,
{
    while let Ok(msg) = input.recv().await {
        let msg = match msg {
            Some(val) => val,
            None => continue,
        };
        let msgs = derive_item.process(&msg);
        let msgs = match msgs {
            Some(val) => val,
            None => continue,
        };
        for msg1 in msgs {
            output
                .send(msg1)
                .await
                .map_err(|e| Error::TokioSynBroadcast(e.to_string()))?
        }
    }
    Ok(())
}
