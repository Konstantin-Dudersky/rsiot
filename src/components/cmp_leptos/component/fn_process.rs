use gloo::storage::{LocalStorage, Storage};
use leptos::prelude::*;
use tokio::task::JoinSet;
use tracing::debug;

use crate::{
    executor::CmpInOut,
    message::{system_messages::*, *},
};

use super::{
    super::{Error, GlobalState, Result},
    Config,
};

pub async fn fn_process<TMsg, TView, TIntoView>(
    config: Config<TView, TIntoView>,
    in_out: CmpInOut<TMsg>,
) -> Result
where
    TMsg: MsgDataBound + 'static,
    TView: Fn() -> TIntoView + 'static,
    TIntoView: IntoView,
{
    let gs = GlobalState::<TMsg> {
        hostname: config.hostname,
        input: RwSignal::new(None),
        output: RwSignal::new(None),
        cache: in_out.cache.clone(),
        auth_perm: RwSignal::new(AuthPermissions::NoAccess),
    };

    // Монтируем корневой компонент
    let gs_clone = gs.clone();
    mount_to_body(move || {
        view! {
            <RootComponent body_component = config.body_component global_state=gs_clone />
        }
    });
    debug!("Leptos app mounted");

    let mut task_set: JoinSet<Result> = JoinSet::new();
    task_set.spawn_local(task_input(in_out.clone(), gs.clone()));
    task_set.spawn_local(task_output(in_out, gs.clone()));
    while let Some(task_result) = task_set.join_next().await {
        task_result??
    }
    Ok(())
}

async fn task_input<TMsg>(mut input: CmpInOut<TMsg>, global_state: GlobalState<TMsg>) -> Result
where
    TMsg: MsgDataBound + 'static,
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

async fn task_output<TMsg>(output: CmpInOut<TMsg>, global_state: GlobalState<TMsg>) -> Result
where
    TMsg: MsgDataBound + 'static,
{
    if let Some(msg) = try_to_find_token() {
        output.send_output(msg).await.map_err(Error::CmpOutput)?;
    }

    let (tx, mut rx) = tokio::sync::mpsc::channel(100);

    Effect::new(move |_| {
        let msg = global_state.output.get();
        if let Some(msg) = msg {
            tx.blocking_send(msg)
                .map_err(|e| Error::TokioMpscSend(e.to_string()))?;
        }
        Ok(()) as Result
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

/// Корневой компонент
#[component]
fn RootComponent<TMsg, TView, TIntoView>(
    body_component: TView,
    global_state: GlobalState<TMsg>,
) -> impl IntoView
where
    TMsg: MsgDataBound + 'static,
    TView: Fn() -> TIntoView,
    TIntoView: IntoView,
{
    provide_context(global_state);

    view! {
        { body_component() }
    }
}
