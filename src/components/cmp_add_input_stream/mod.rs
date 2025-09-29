//! Компонент для добавления сообщений из побочного потока

use async_trait::async_trait;
use tokio::sync::broadcast;

use crate::{
    executor::{CmpInOut, Component, ComponentError, IComponentProcess},
    message::*,
};

/// Название компонента
pub const CMP_NAME: &str = "cmp_add_input_stream";

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
        msg_bus: CmpInOut<TMsg>,
    ) -> Result<(), ComponentError> {
        let output = msg_bus.init(CMP_NAME).output();

        while let Ok(msg) = config.channel.recv().await {
            output
                .send(msg)
                .await
                .map_err(|err| ComponentError::Execution(err.to_string()))?;
        }
        Ok(())
    }
}

/// Компонент cmp_add_input_stream
pub type Cmp<TMsg> = Component<Config<TMsg>, TMsg>;
