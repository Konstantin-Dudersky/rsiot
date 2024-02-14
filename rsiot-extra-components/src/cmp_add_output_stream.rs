//! Компонент для отправки сообщений в побочный потока

use async_trait::async_trait;

use rsiot_component_core::{
    Cache, CmpOutput, Component, ComponentError, ComponentInput, IComponentProcess,
};
use rsiot_messages_core::IMessage;

use super::cmpbase_mpsc_to_many_mpsc;

/// Настройки
#[derive(Debug)]
pub struct Cfg<TMessage>
where
    TMessage: IMessage,
{
    pub channel: CmpOutput<TMessage>,
}

#[cfg(not(feature = "single-thread"))]
#[async_trait]
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
        cmpbase_mpsc_to_many_mpsc::new(input, vec![output, config.channel]).await;
        Ok(())
    }
}

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
        cmpbase_mpsc_to_many_mpsc::new(input, vec![output, config.channel]).await;
        Ok(())
    }
}

pub type Cmp<TMsg> = Component<Cfg<TMsg>, TMsg>;
