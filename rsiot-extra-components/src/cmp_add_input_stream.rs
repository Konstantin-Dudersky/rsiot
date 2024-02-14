//! Компонент для добавления сообщений из побочного потока

use async_trait::async_trait;
use tokio::task::JoinSet;

use rsiot_component_core::{
    Cache, CmpOutput, Component, ComponentError, ComponentInput, IComponentProcess,
};
use rsiot_messages_core::IMessage;

async fn task_subscription<TMessage>(
    mut input: ComponentInput<TMessage>,
    output: CmpOutput<TMessage>,
) -> Result<(), ComponentError>
where
    TMessage: IMessage,
{
    while let Ok(msg) = input.recv().await {
        output
            .send(msg)
            .await
            .map_err(|err| ComponentError::Execution(err.to_string()))?;
    }
    Ok(())
}

/// Настройки
#[derive(Debug)]
pub struct Cfg<TMessage> {
    pub channel: ComponentInput<TMessage>,
}

/// Компонент для добавления сообщений из побочного потока
#[cfg(not(feature = "single-thread"))]
#[async_trait()]
impl<TMsg> IComponentProcess<Cfg<TMsg>, TMsg> for Component<Cfg<TMsg>, TMsg>
where
    TMsg: IMessage + 'static,
{
    async fn process(
        &self,
        config: Cfg<TMsg>,
        input: ComponentInput<TMsg>,
        output: CmpOutput<TMsg>,
        _cache: Cache<TMsg>,
    ) -> Result<(), ComponentError> {
        let mut task_set: JoinSet<Result<(), ComponentError>> = JoinSet::new();

        task_set.spawn(task_subscription(input, output.clone()));
        task_set.spawn(task_subscription(config.channel, output.clone()));

        while let Some(res) = task_set.join_next().await {
            res.map_err(|err| ComponentError::Execution(err.to_string()))??;
        }
        Ok(())
    }
}

/// Компонент для добавления сообщений из побочного потока
#[cfg(feature = "single-thread")]
#[async_trait(?Send)]
impl<TMsg> IComponentProcess<Cfg<TMsg>, TMsg> for Component<Cfg<TMsg>, TMsg>
where
    TMsg: IMessage + 'static,
{
    async fn process(
        &self,
        config: Cfg<TMsg>,
        input: ComponentInput<TMsg>,
        output: CmpOutput<TMsg>,
        _cache: Cache<TMsg>,
    ) -> Result<(), ComponentError> {
        let mut task_set: JoinSet<Result<(), ComponentError>> = JoinSet::new();

        task_set.spawn(task_subscription(input, output.clone()));
        task_set.spawn(task_subscription(config.channel, output.clone()));

        while let Some(res) = task_set.join_next().await {
            res.map_err(|err| ComponentError::Execution(err.to_string()))??;
        }
        Ok(())
    }
}

pub type Cmp<TMsg> = Component<Cfg<TMsg>, TMsg>;
