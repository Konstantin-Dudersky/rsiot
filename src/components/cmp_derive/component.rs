use async_trait::async_trait;

use crate::{
    executor::{CmpInOut, Component, ComponentError, IComponentProcess},
    message::*,
};

use super::{Config, fn_process::fn_process};

pub const COMPONENT_NAME: &str = "cmp_derive";

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
        fn_process(input, output, config)
            .await
            .map_err(|e| ComponentError::Execution(e.to_string()))
    }
}

/// Компонент cmp_derive
pub type Cmp<TMsg> = Component<Config<TMsg>, TMsg>;
