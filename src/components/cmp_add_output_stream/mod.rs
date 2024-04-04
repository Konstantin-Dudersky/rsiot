//! Компонент для отправки сообщений в побочный потока

use async_trait::async_trait;
use tokio::sync::mpsc;

use crate::{
    executor::{CmpInOut, Component, ComponentError, IComponentProcess},
    message::*,
};

/// Настройки
#[derive(Debug)]
pub struct Cfg<TMessage> {
    /// Внешний канал mpsc, в который пересылаются исходящие сообщения
    pub channel: mpsc::Sender<Message<TMessage>>,
}

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMsg> IComponentProcess<Cfg<TMsg>, TMsg> for Component<Cfg<TMsg>, TMsg>
where
    TMsg: MsgDataBound + 'static,
{
    async fn process(
        &self,
        config: Cfg<TMsg>,
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
pub type Cmp<TMsg> = Component<Cfg<TMsg>, TMsg>;
