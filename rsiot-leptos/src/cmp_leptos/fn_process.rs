use leptos::*;
use rsiot_component_core::{Cache, ComponentInput, ComponentOutput};
use rsiot_messages_core::{msg_meta::ServiceId, IMessage};
use tokio::task::JoinSet;
use tracing::debug;

use crate::GlobalState;

use super::Config;

pub async fn fn_process<TMsg, TView, TIntoView>(
    config: Config<TView, TIntoView>,
    input: ComponentInput<TMsg>,
    output: ComponentOutput<TMsg>,
    cache: Cache<TMsg>,
) -> crate::Result<TMsg>
where
    TMsg: IMessage + 'static,
    TView: Fn() -> TIntoView + 'static,
    TIntoView: IntoView,
{
    let component_id = ServiceId::new("cmp_leptos");

    provide_context(GlobalState::<TMsg> {
        service_id: component_id,
        hostname: config.hostname,
        input: create_rw_signal(None),
        output: create_rw_signal(None),
        cache,
    });
    let gs = use_context::<GlobalState<TMsg>>().expect("No global state");

    mount_to_body(config.body_component);
    debug!("Leptos app mounted");

    let mut task_set: JoinSet<crate::Result<TMsg>> = JoinSet::new();
    task_set.spawn_local(task_input(input, gs.clone()));
    task_set.spawn_local(task_output(output, gs));
    while let Some(task_result) = task_set.join_next().await {
        task_result??
    }
    Ok(())
}

async fn task_input<TMsg>(
    mut input: ComponentInput<TMsg>,
    gs: GlobalState<TMsg>,
) -> crate::Result<TMsg>
where
    TMsg: IMessage,
{
    while let Ok(msg) = input.recv().await {
        gs.input.set(Some(msg));
    }
    Ok(())
}

async fn task_output<TMsg>(
    output: ComponentOutput<TMsg>,
    gs: GlobalState<TMsg>,
) -> crate::Result<TMsg>
where
    TMsg: IMessage,
{
    let (tx, mut rx) = tokio::sync::mpsc::channel(100);

    create_effect(move |_| {
        let msg = gs.output.get();
        if let Some(msg) = msg {
            tx.blocking_send(msg)?;
        }
        Ok(()) as crate::Result<TMsg>
    });

    while let Some(msg) = rx.recv().await {
        output.send(msg).await?;
    }

    Ok(())
}
