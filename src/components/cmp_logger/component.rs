use crate::{
    executor::{CmpInOut, Component, ComponentError, IComponentProcess},
    message::MsgDataBound,
};
use async_trait::async_trait;

use super::{Config, fn_process::fn_process};

/// Название компонента
pub const COMPONENT_NAME: &str = "cmp_logger";

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMsg> IComponentProcess<Config<TMsg>, TMsg> for Component<Config<TMsg>, TMsg>
where
    TMsg: 'static + MsgDataBound,
{
    async fn process(
        &self,
        config: Config<TMsg>,
        in_out: CmpInOut<TMsg>,
    ) -> Result<(), ComponentError> {
        let input = in_out.msgbus_input(COMPONENT_NAME);
        fn_process(config, input).await?;
        Ok(())
    }
}

/// Компонент cmp_logger
pub type Cmp<TMsg> = Component<Config<TMsg>, TMsg>;
