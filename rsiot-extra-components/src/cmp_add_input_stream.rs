//! Компонент для добавления сообщений из побочного потока

use tokio::task::JoinSet;

use rsiot_component_core::{Component, Input, Output};
use rsiot_messages_core::IMessage;

async fn fn_process<TMessage>(
    mut input: Input<TMessage>,
    output: Output<TMessage>,
    mut config: Config<TMessage>,
) where
    TMessage: IMessage + 'static,
{
    let mut task_set = JoinSet::new();

    let output_clone = output.clone();
    task_set.spawn(async move {
        while let Ok(msg) = input.recv().await {
            output_clone.send(msg).await.unwrap();
        }
    });

    task_set.spawn(async move {
        while let Ok(msg) = config.channel.recv().await {
            output.send(msg).await.unwrap();
        }
    });

    while let Some(res) = task_set.join_next().await {
        res.unwrap();
    }
}

/// Настройки
#[derive(Debug)]
pub struct Config<TMessage> {
    pub channel: Input<TMessage>,
}

/// Компонент для добавления сообщений из побочного потока
pub fn new<TMessage>(config: Config<TMessage>) -> Box<Component<TMessage, Config<TMessage>>>
where
    TMessage: IMessage + 'static,
{
    let cmp = Component::new(config, fn_process);
    Box::new(cmp)
}
