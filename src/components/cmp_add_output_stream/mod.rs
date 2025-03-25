//! Компонент для отправки сообщений в побочный потока

use async_trait::async_trait;
use tokio::sync::mpsc;

use crate::{
    executor::{CmpInOut, Component, ComponentError, IComponentProcess},
    message::*,
};

/// Настройки компонента cmp_add_output_stream
#[derive(Debug)]
pub struct Config<TMessage> {
    /// Внешний канал mpsc, в который пересылаются исходящие сообщения
    pub channel: mpsc::Sender<Message<TMessage>>,
}

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMsg> IComponentProcess<Config<TMsg>, TMsg> for Component<Config<TMsg>, TMsg>
where
    TMsg: MsgDataBound + 'static,
{
    async fn process(
        &self,
        config: Config<TMsg>,
        in_out: CmpInOut<TMsg>,
    ) -> Result<(), ComponentError> {
        let mut in_out =
            in_out.clone_with_new_id("cmp_add_output_stream", AuthPermissions::FullAccess);
        while let Ok(msg) = in_out.recv_input().await {
            config.channel.send(msg.clone()).await.unwrap();
        }
        Ok(())
    }
}

/// Компонент cmp_add_output_stream
pub type Cmp<TMsg> = Component<Config<TMsg>, TMsg>;
