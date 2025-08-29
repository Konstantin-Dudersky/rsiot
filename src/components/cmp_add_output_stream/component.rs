use async_trait::async_trait;

use crate::{
    executor::{CmpInOut, Component, ComponentError, IComponentProcess},
    message::*,
};

use super::{Config, Error};

/// Название компонента
pub const COMPONENT_NAME: &str = "CMP_TEMPLATE";

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
        let mut in_out = in_out.clone_with_new_id(COMPONENT_NAME, AuthPermissions::FullAccess);
        while let Ok(msg) = in_out.recv_input().await {
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
