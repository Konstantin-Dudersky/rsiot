//! Компонент для добавления сообщений из побочного потока

use async_trait::async_trait;
use tokio::sync::broadcast;

use rsiot_component_core::{CmpInOut, Component, ComponentError, IComponentProcess};
use rsiot_messages_core::*;

/// Настройки
#[derive(Debug)]
pub struct Cfg<TMessage> {
    pub channel: broadcast::Receiver<Message<TMessage>>,
}

/// Компонент для добавления сообщений из побочного потока
#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMsg> IComponentProcess<Cfg<TMsg>, TMsg> for Component<Cfg<TMsg>, TMsg>
where
    TMsg: MsgDataBound + 'static,
{
    async fn process(
        &self,
        mut config: Cfg<TMsg>,
        in_out: CmpInOut<TMsg>,
    ) -> Result<(), ComponentError> {
        while let Ok(msg) = config.channel.recv().await {
            in_out
                .send_output(msg)
                .await
                .map_err(|err| ComponentError::Execution(err.to_string()))?;
        }
        Ok(())
    }
}

pub type Cmp<TMsg> = Component<Cfg<TMsg>, TMsg>;
