use async_trait::async_trait;

use crate::{
    executor::{CmpInOut, Component, ComponentError, IComponentProcess},
    message::MsgDataBound,
};

use super::{config::Config, fn_process::fn_process};

/// Название компонента
pub const COMPONENT_NAME: &str = "cmp_modbus_client";

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMessage> IComponentProcess<Config<TMessage>, TMessage>
    for Component<Config<TMessage>, TMessage>
where
    TMessage: MsgDataBound + 'static,
{
    async fn process(
        &self,
        config: Config<TMessage>,
        in_out: CmpInOut<TMessage>,
    ) -> Result<(), ComponentError> {
        let (input, output) = in_out.msgbus_input_output(COMPONENT_NAME);
        fn_process(config, input, output).await?;
        Ok(())
    }
}

/// Компонент cmp_modbus_client
pub type Cmp<TMessage> = Component<Config<TMessage>, TMessage>;
