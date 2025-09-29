use async_trait::async_trait;

use crate::{
    executor::{CmpInOut, Component, ComponentError, IComponentProcess},
    message::*,
};

use super::{Config, Error};

/// Название компонента
pub const CMP_NAME: &str = "cmp_add_output_stream";

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMsg> IComponentProcess<Config<TMsg>, TMsg> for Component<Config<TMsg>, TMsg>
where
    TMsg: MsgDataBound + 'static,
{
    async fn process(
        &self,
        config: Config<TMsg>,
        msg_bus: CmpInOut<TMsg>,
    ) -> Result<(), ComponentError> {
        let mut input = msg_bus.init(CMP_NAME).input();

        while let Ok(msg) = input.recv().await {
            config
                .channel
                .send(msg.clone())
                .await
                .map_err(|_| Error::TokioSyncMpscSend)?;
        }
        Ok(())
    }
}

/// Компонент cmp_add_output_stream
pub type Cmp<TMsg> = Component<Config<TMsg>, TMsg>;
