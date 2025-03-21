use gloo::storage::{LocalStorage, Storage};
use leptos::prelude::*;
use reactive_stores::Store;
use tokio::{sync::mpsc, task::JoinSet};
use tracing::debug;

use crate::{
    executor::{join_set_spawn, CmpInOut},
    message::{system_messages::*, *},
};

use super::{
    super::{utils, Error, GlobalState, Result},
    Config, StoreBound,
};

pub async fn fn_process<TMsg, TView, TIntoView, TService, TInputStore, TOutputStore>(
    config: Config<TMsg, TView, TIntoView, TInputStore, TOutputStore>,
    in_out: CmpInOut<TMsg, TService>,
) -> Result
where
    TMsg: MsgDataBound + 'static,
    TView: Fn() -> TIntoView + 'static,
    TIntoView: IntoView,
    TService: ServiceBound + 'static,
    TInputStore: StoreBound + 'static,
    TOutputStore: StoreBound + 'static,
{
    let hostname = utils::define_hostname().unwrap();

    let gs = GlobalState::<TMsg> {
        hostname,
        input: RwSignal::new(None),
        output: RwSignal::new(None),
        cache: in_out.cache.clone(),
        auth_perm: RwSignal::new(AuthPermissions::NoAccess),
    };

    // Монтируем корневой компонент
    let input_store = Store::new(config.input_store);
    let input_store_clone = input_store;
    let output_store = Store::new(config.output_store);
    let output_store_clone = output_store;
    mount_to_body(move || {
        view! {
            <RootComponent
                body_component = config.body_component
                input_store=input_store_clone
                output_store=output_store_clone
            />
        }
    });
    debug!("Leptos app mounted");

    let mut task_set: JoinSet<Result> = JoinSet::new();

    let task = task_input(in_out.clone(), config.fn_input, input_store);
    join_set_spawn(&mut task_set, task);

    let task = task_output(in_out.clone(), config.fn_output, output_store);
    join_set_spawn(&mut task_set, task);

    while let Some(task_result) = task_set.join_next().await {
        task_result??
    }
    Ok(())
}

async fn task_input<TMsg, TService, TInputStore>(
    mut msg_bus: CmpInOut<TMsg, TService>,
    fn_input: fn(&Message<TMsg>, &Store<TInputStore>),
    input_store: Store<TInputStore>,
) -> Result
where
    TMsg: MsgDataBound + 'static,
    TService: ServiceBound + 'static,
    TInputStore: StoreBound + 'static,
{
    while let Ok(msg) = msg_bus.recv_input().await {
        (fn_input)(&msg, &input_store);
    }
    Ok(())
}

async fn task_output<TMsg, TService, TOutputStore>(
    msg_bus: CmpInOut<TMsg, TService>,
    fn_output: fn(Store<TOutputStore>, mpsc::Sender<TMsg>),
    output_store: Store<TOutputStore>,
) -> Result
where
    TMsg: MsgDataBound + 'static,
    TService: ServiceBound + 'static,
    TOutputStore: StoreBound + 'static,
{
    let (tx, mut rx) = mpsc::channel(100);

    (fn_output)(output_store, tx);

    while let Some(msg) = rx.recv().await {
        let msg = Message::new_custom(msg);
        msg_bus.send_output(msg).await.map_err(Error::CmpOutput)?;
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
            let msg = Message::new(MsgData::System(System::AuthRequestByToken(value)));
            Some(msg)
        }
        _ => None,
    }
}

/// Корневой компонент
#[component]
fn RootComponent<TView, TIntoView, TInputStore, TOutputStore>(
    body_component: TView,
    input_store: Store<TInputStore>,
    output_store: Store<TOutputStore>,
) -> impl IntoView
where
    TView: Fn() -> TIntoView,
    TIntoView: IntoView,
    TInputStore: StoreBound + 'static,
    TOutputStore: StoreBound + 'static,
{
    provide_context(input_store);
    provide_context(output_store);

    view! {
        { body_component() }
    }
}
