use gloo::storage::{LocalStorage, Storage};
use leptos::*;
use tokio::task::JoinSet;
use tracing::debug;

use rsiot_component_core::CmpInOut;
use rsiot_messages_core::{system_messages::*, *};

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
        auth_perm: create_rw_signal(AuthPermissions::NoAccess),
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

async fn task_input<TMsg>(
    mut input: CmpInOut<TMsg>,
    global_state: GlobalState<TMsg>,
) -> crate::Result
where
    TMsg: MsgDataBound,
{
    while let Ok(msg) = input.recv_input().await {
        // Разрешения
        match &msg.data {
            MsgData::System(System::AuthResponseOk(value)) => {
                global_state.auth_perm.set(value.perm);
                LocalStorage::set(&msg.key, &msg)?;
            }
            MsgData::System(System::AuthResponseErr(_)) => {
                global_state.auth_perm.set(AuthPermissions::NoAccess);
                LocalStorage::delete(&msg.key);
            }
            _ => (),
        }
        // Пересылаем сообщение в сигнал
        global_state.input.set(Some(msg));
    }
    Ok(())
}

async fn task_output<TMsg>(output: CmpInOut<TMsg>, gs: GlobalState<TMsg>) -> crate::Result
where
    TMsg: MsgDataBound,
{
    if let Some(msg) = try_to_find_token() {
        output.send_output(msg).await.map_err(Error::CmpOutput)?;
    }

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

/// Пробуем найти токен в LocalStorage.
///
/// Если токен присутствует, отправляем запрос на проверку токена
fn try_to_find_token<TMsg>() -> Option<Message<TMsg>>
where
    TMsg: MsgDataBound,
{
    let msg: Message<TMsg> = LocalStorage::get("System-AuthResponseOk").ok()?;
    match msg.data {
        MsgData::System(System::AuthResponseOk(value)) => {
            let value = AuthRequestByToken { token: value.token };
            let msg = message_new!("System-AuthRequestByToken::value");
            Some(msg)
        }
        _ => None,
    }
}
