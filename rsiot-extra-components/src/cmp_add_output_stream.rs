//! Компонент для отправки сообщений в побочный потока

use async_trait::async_trait;

use rsiot_component_core::{
    cmp_set_component_id, Cache, CmpInput, CmpOutput, Component, ComponentError, IComponentProcess,
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

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMsg> IComponentProcess<Cfg<TMsg>, TMsg> for Component<Cfg<TMsg>, TMsg>
where
    TMsg: IMessage + 'static,
{
    async fn process(
        &self,
        config: Cfg<TMsg>,
        mut input: CmpInput<TMsg>,
        mut output: CmpOutput<TMsg>,
        _cache: Cache<TMsg>,
    ) -> Result<(), ComponentError> {
        cmp_set_component_id(&mut input, &mut output, "cmp_add_output_stream");
        cmpbase_mpsc_to_many_mpsc::new(input, vec![output, config.channel]).await;
        Ok(())
    }
}

pub type Cmp<TMsg> = Component<Cfg<TMsg>, TMsg>;
