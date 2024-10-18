//! Компонент для добавления сообщений из побочного потока

use async_trait::async_trait;
use tokio::sync::broadcast;

use crate::{
    executor::{CmpInOut, Component, ComponentError, IComponentProcess},
    message::*,
};

/// Настройки компонента cmp_add_input_stream
#[derive(Debug)]
pub struct Config<TMessage> {
    /// Внешний канал broadcast, на который происходит подписка
    pub channel: broadcast::Receiver<Message<TMessage>>,
}

/// Компонент для добавления сообщений из побочного потока
#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMsg> IComponentProcess<Config<TMsg>, TMsg> for Component<Config<TMsg>, TMsg>
where
    TMsg: MsgDataBound + 'static,
{
    async fn process(
        &self,
        mut config: Config<TMsg>,
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

/// Компонент cmp_add_input_stream
pub type Cmp<TMsg> = Component<Config<TMsg>, TMsg>;
