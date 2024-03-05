use leptos::*;
use rsiot_component_core::{Cache, CmpInOut};
use rsiot_messages_core::MsgDataBound;
use tokio::task::JoinSet;
use tracing::debug;

use crate::{Error, GlobalState};

use super::Config;

pub async fn fn_process<TMsg, TView, TIntoView>(
    config: Config<TView, TIntoView>,
    in_out: CmpInOut<TMsg>,
) -> crate::Result
where
    TMsg: MsgDataBound + 'static,
    TView: Fn() -> TIntoView + 'static,
    TIntoView: IntoView,
{
    provide_context(GlobalState::<TMsg> {
        hostname: config.hostname,
        input: create_rw_signal(None),
        output: create_rw_signal(None),
        cache: in_out.cache.clone(),
    });
    let gs = use_context::<GlobalState<TMsg>>().expect("No global state");

    mount_to_body(config.body_component);
    debug!("Leptos app mounted");

    let mut task_set: JoinSet<crate::Result> = JoinSet::new();
    task_set.spawn_local(task_input(in_out.clone(), gs.clone()));
    task_set.spawn_local(task_output(in_out, gs));
    while let Some(task_result) = task_set.join_next().await {
        task_result??
    }
    Ok(())
}

async fn task_input<TMsg>(mut input: CmpInOut<TMsg>, gs: GlobalState<TMsg>) -> crate::Result
where
    TMsg: MsgDataBound,
{
    while let Ok(msg) = input.recv_input().await {
        let msg = match msg {
            Some(val) => val,
            None => continue,
        };
        gs.input.set(Some(msg));
    }
    Ok(())
}

async fn task_output<TMsg>(output: CmpInOut<TMsg>, gs: GlobalState<TMsg>) -> crate::Result
where
    TMsg: MsgDataBound,
{
    let (tx, mut rx) = tokio::sync::mpsc::channel(100);

    create_effect(move |_| {
        let msg = gs.output.get();
        if let Some(msg) = msg {
            tx.blocking_send(msg)
                .map_err(|e| Error::TokioMpscSend(e.to_string()))?;
        }
        Ok(()) as crate::Result
    });

    while let Some(msg) = rx.recv().await {
        output.send_output(msg).await.map_err(Error::CmpOutput)?;
    }

    Ok(())
}
