use tokio::task::JoinSet;

use crate::executor::{join_set_spawn, CmpInOut};
use crate::message::*;

use super::{Config, DeriveItemProcess, Error};

pub async fn fn_process<TMsg, TService>(
    in_out: CmpInOut<TMsg, TService>,
    config: Config<TMsg>,
) -> super::Result<()>
where
    TMsg: MsgDataBound + 'static,
    TService: ServiceBound + 'static,
{
    let mut task_set = JoinSet::new();

    for item in config.derive_items {
        join_set_spawn(
            &mut task_set,
            task_process_derive_item(in_out.clone(), item),
        );
    }

    while let Some(res) = task_set.join_next().await {
        res??
    }
    Ok(())
}

async fn task_process_derive_item<TMsg, TService>(
    mut in_out: CmpInOut<TMsg, TService>,
    mut derive_item: Box<dyn DeriveItemProcess<TMsg>>,
) -> super::Result<()>
where
    TMsg: MsgDataBound,
    TService: ServiceBound,
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
