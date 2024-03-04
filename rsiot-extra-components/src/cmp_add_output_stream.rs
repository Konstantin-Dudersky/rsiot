//! Компонент для отправки сообщений в побочный потока

use async_trait::async_trait;
use tokio::sync::mpsc;

use rsiot_component_core::{Cache, CmpInOut, Component, ComponentError, IComponentProcess};
use rsiot_messages_core::*;

/// Настройки
#[derive(Debug)]
pub struct Cfg<TMessage> {
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
        _cache: Cache<TMsg>,
    ) -> Result<(), ComponentError> {
        let mut in_out =
            in_out.clone_with_new_id("cmp_add_output_stream", AuthPermissions::FullAccess);
        while let Ok(msg) = in_out.recv_input().await {
            let msg = match msg {
                Some(val) => val,
                None => continue,
            };
            config.channel.send(msg.clone()).await.unwrap();
        }
        Ok(())
    }
}

pub type Cmp<TMsg> = Component<Cfg<TMsg>, TMsg>;
