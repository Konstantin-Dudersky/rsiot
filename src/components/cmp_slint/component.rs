use async_trait::async_trait;
use slint::ComponentHandle;

use crate::{
    executor::{CmpInOut, Component, ComponentError, IComponentProcess},
    message::MsgDataBound,
};

use super::{Config, fn_process::fn_process};

/// Название компонента
pub const COMPONENT_NAME: &str = "cmp_slint";

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMainWindow, TMsg> IComponentProcess<Config<TMsg, TMainWindow>, TMsg>
    for Component<Config<TMsg, TMainWindow>, TMsg>
where
    TMsg: MsgDataBound + 'static,
    Self: Sync,
    TMainWindow: ComponentHandle + 'static,
{
    async fn process(
        &self,
        config: Config<TMsg, TMainWindow>,
        input: CmpInOut<TMsg>,
    ) -> Result<(), ComponentError> {
        let (input, output) = input.msgbus_input_output(COMPONENT_NAME);
        fn_process(config, input, output).await?;
        Ok(())
    }
}

/// Компонент cmp_slint
pub type Cmp<TMsg, TMainWindow> = Component<Config<TMsg, TMainWindow>, TMsg>;
