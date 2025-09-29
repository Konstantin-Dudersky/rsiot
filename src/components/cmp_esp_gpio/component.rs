use async_trait::async_trait;

use crate::{
    executor::{CmpInOut, Component, ComponentError, IComponentProcess},
    message::MsgDataBound,
};

use super::{config::Config, fn_process::fn_process};

/// Название компонента
pub const COMPONENT_NAME: &str = "cmp_esp_gpio";

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
        let (input, output) = in_out.msgbus_input_output(COMPONENT_NAME);
        fn_process(config, input, output).await?;
        Ok(())
    }
}

/// Компонент cmp_esp_wifi
pub type Cmp<TMsg> = Component<Config<TMsg>, TMsg>;
