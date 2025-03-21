use slint::ComponentHandle;
use tokio::{sync::mpsc, task::JoinSet};

use crate::{
    executor::CmpInOut,
    message::{MsgDataBound, ServiceBound},
};

use super::{Config, Result};

pub async fn fn_process<TMainWindow, TMsg, TService>(
    config: Config<TMsg, TMainWindow>,
    input: CmpInOut<TMsg, TService>,
) -> Result<()>
where
    TMsg: MsgDataBound + 'static,
    TService: ServiceBound + 'static,
    TMainWindow: ComponentHandle + 'static,
{
    let mut task_set = JoinSet::new();

    task_set.spawn(fn_input(config.clone(), input.clone()));
    task_set.spawn(fn_output(config.clone(), input));

    while let Some(res) = task_set.join_next().await {
        res.unwrap();
    }
    Ok(())
}

async fn fn_input<TMainWindow, TMsg, TService>(
    config: Config<TMsg, TMainWindow>,
    mut input: CmpInOut<TMsg, TService>,
) where
    TMsg: MsgDataBound + 'static,
    TService: ServiceBound + 'static,
    TMainWindow: ComponentHandle + 'static,
{
    while let Ok(msg) = input.recv_input().await {
        let window = config.instance.lock().await;

        window
            .upgrade_in_event_loop(move |h| (config.fn_input)(msg, h))
            .unwrap();
    }
}

async fn fn_output<TMainWindow, TMsg, TService>(
    config: Config<TMsg, TMainWindow>,
    input: CmpInOut<TMsg, TService>,
) where
    TMsg: MsgDataBound + 'static,
    TService: ServiceBound + 'static,
    TMainWindow: ComponentHandle + 'static,
{
    let (tx, mut rx) = mpsc::channel(100);

    {
        let lock = config.instance.lock().await;
        lock.upgrade_in_event_loop(move |h| (config.fn_output)(h, tx))
            .unwrap();
    }

    while let Some(msg) = rx.recv().await {
        input.send_output(msg).await.unwrap();
    }
}
