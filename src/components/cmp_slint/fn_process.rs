use slint::ComponentHandle;
use tokio::{sync::mpsc, task::JoinSet};

use crate::{executor::CmpInOut, message::MsgDataBound};

use super::{Config, Result};

pub async fn fn_process<TMainWindow, TMsg>(
    config: Config<TMainWindow, TMsg>,
    input: CmpInOut<TMsg>,
) -> Result<()>
where
    TMsg: MsgDataBound + 'static,
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

async fn fn_input<TMainWindow, TMsg>(config: Config<TMainWindow, TMsg>, mut input: CmpInOut<TMsg>)
where
    TMsg: MsgDataBound + 'static,
    TMainWindow: ComponentHandle,
{
    while let Ok(msg) = input.recv_input().await {
        let lock = config.instance.lock().await;
        (config.fn_input)(msg, lock);
    }
}

async fn fn_output<TMainWindow, TMsg>(config: Config<TMainWindow, TMsg>, input: CmpInOut<TMsg>)
where
    TMsg: MsgDataBound + 'static,
    TMainWindow: ComponentHandle,
{
    let (tx, mut rx) = mpsc::channel(100);

    {
        let lock = config.instance.lock().await;
        (config.fn_output)(lock, tx);
    }

    while let Some(msg) = rx.recv().await {
        input.send_output(msg).await.unwrap();
    }
}
