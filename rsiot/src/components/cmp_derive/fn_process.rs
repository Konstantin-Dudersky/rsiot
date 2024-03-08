use tokio::task::JoinSet;

use crate::executor::CmpInOut;
use crate::message::*;

use super::{Config, DeriveItemProcess, Error};

pub async fn fn_process<TMsg>(in_out: CmpInOut<TMsg>, config: Config<TMsg>) -> super::Result<()>
where
    TMsg: MsgDataBound + 'static,
{
    let mut task_set = JoinSet::new();

    for item in config.derive_items {
        task_set.spawn(task_process_derive_item(in_out.clone(), item));
    }

    while let Some(res) = task_set.join_next().await {
        res??
    }
    Ok(())
}

async fn task_process_derive_item<TMsg>(
    mut in_out: CmpInOut<TMsg>,
    mut derive_item: Box<dyn DeriveItemProcess<TMsg>>,
) -> super::Result<()>
where
    TMsg: MsgDataBound,
{
    while let Ok(msg) = in_out.recv_input().await {
        let msgs = derive_item.process(&msg);
        let Some(msgs) = msgs else { continue };
        for msg in msgs {
            in_out
                .send_output(msg)
                .await
                .map_err(|e| Error::TokioSynBroadcast(e.to_string()))?
        }
    }
    Ok(())
}
