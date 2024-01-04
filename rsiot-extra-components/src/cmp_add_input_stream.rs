//! Компонент для добавления сообщений из побочного потока

use tokio::task::JoinSet;

use rsiot_component_core::{Cache, Component, ComponentError, ComponentInput, ComponentOutput};
use rsiot_messages_core::IMessage;

async fn fn_process<TMessage>(
    mut input: ComponentInput<TMessage>,
    output: ComponentOutput<TMessage>,
    mut config: Config<TMessage>,
    _cache: Cache<TMessage>,
) -> Result<(), ComponentError>
where
    TMessage: IMessage + 'static,
{
    let mut task_set: JoinSet<Result<(), ComponentError>> = JoinSet::new();

    let output_clone = output.clone();
    task_set.spawn(async move {
        while let Ok(msg) = input.recv().await {
            output_clone
                .send(msg)
                .await
                .map_err(|err| ComponentError::Execution(err.to_string()))?;
        }
        Ok(())
    });

    task_set.spawn(async move {
        while let Ok(msg) = config.channel.recv().await {
            output
                .send(msg)
                .await
                .map_err(|err| ComponentError::Execution(err.to_string()))?;
        }
        Ok(())
    });

    while let Some(res) = task_set.join_next().await {
        res.map_err(|err| ComponentError::Execution(err.to_string()))??;
    }
    Ok(())
}

/// Настройки
#[derive(Debug)]
pub struct Config<TMessage> {
    pub channel: ComponentInput<TMessage>,
}

/// Компонент для добавления сообщений из побочного потока
pub fn new<TMessage>(config: Config<TMessage>) -> Box<Component<TMessage, Config<TMessage>>>
where
    TMessage: IMessage + 'static,
{
    let cmp = Component::new(config, fn_process);
    Box::new(cmp)
}
