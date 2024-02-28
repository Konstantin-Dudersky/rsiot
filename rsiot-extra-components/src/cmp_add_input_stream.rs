//! Компонент для добавления сообщений из побочного потока

use async_trait::async_trait;
use tokio::task::JoinSet;

use rsiot_component_core::{
    cmp_set_component_name, Cache, CmpInput, CmpOutput, Component, ComponentError,
    IComponentProcess,
};
use rsiot_messages_core::MsgDataBound;

async fn task_subscription<TMessage>(
    mut input: CmpInput<TMessage>,
    output: CmpOutput<TMessage>,
) -> Result<(), ComponentError>
where
    TMessage: MsgDataBound,
{
    while let Ok(msg) = input.recv().await {
        let msg = match msg {
            Some(val) => val,
            None => continue,
        };
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
    pub channel: CmpInput<TMessage>,
}

/// Компонент для добавления сообщений из побочного потока
#[cfg(not(feature = "single-thread"))]
#[async_trait()]
impl<TMsg> IComponentProcess<Cfg<TMsg>, TMsg> for Component<Cfg<TMsg>, TMsg>
where
    TMsg: MsgDataBound + 'static,
{
    async fn process(
        &self,
        config: Cfg<TMsg>,
        mut input: CmpInput<TMsg>,
        mut output: CmpOutput<TMsg>,
        _cache: Cache<TMsg>,
    ) -> Result<(), ComponentError> {
        cmp_set_component_name(&mut input, &mut output, "cmp_add_input_stream");
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
    TMsg: MsgDataBound + 'static,
{
    async fn process(
        &self,
        config: Cfg<TMsg>,
        mut input: CmpInput<TMsg>,
        mut output: CmpOutput<TMsg>,
        _cache: Cache<TMsg>,
    ) -> Result<(), ComponentError> {
        cmp_set_component_name(&mut input, &mut output, "cmp_add_input_stream");
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
