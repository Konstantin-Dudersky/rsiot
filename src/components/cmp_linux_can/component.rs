use async_trait::async_trait;

use crate::{
    components_config::can_general::BufferBound,
    executor::{CmpInOut, CmpResult, Component, IComponentProcess},
    message::MsgDataBound,
};

use super::{config::Config, fn_process::fn_process};

/// Название компонента
pub const COMPONENT_NAME: &str = "cmp_linux_can";

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMsg, TBuffer> IComponentProcess<Config<TMsg, TBuffer>, TMsg>
    for Component<Config<TMsg, TBuffer>, TMsg>
where
    TMsg: MsgDataBound + 'static,
    TBuffer: BufferBound + 'static,
{
    async fn process(&self, config: Config<TMsg, TBuffer>, msg_bus: CmpInOut<TMsg>) -> CmpResult {
        let (input, output) = msg_bus.msgbus_input_output(COMPONENT_NAME);
        fn_process(config, input, output).await?;
        Ok(())
    }
}

/// Компонент cmp_linux_can
pub type Cmp<TMsg, TBuffer> = Component<Config<TMsg, TBuffer>, TMsg>;
