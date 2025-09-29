use async_trait::async_trait;

use crate::{
    executor::{CmpInOut, Component, ComponentError, IComponentProcess},
    message::MsgDataBound,
};

use super::fn_process::fn_process;

/// Название компонента
pub const COMPONENT_NAME: &str = "cmp_surrealdb";

#[cfg_attr(feature = "single-thread", async_trait(?Send))]
#[cfg_attr(not(feature = "single-thread"), async_trait)]
impl<TMsg> IComponentProcess<super::Config<TMsg>, TMsg> for Component<super::Config<TMsg>, TMsg>
where
    TMsg: MsgDataBound + 'static,
{
    async fn process(
        &self,
        config: super::Config<TMsg>,
        input: CmpInOut<TMsg>,
    ) -> Result<(), ComponentError> {
        let (input, output) = input.msgbus_input_output(COMPONENT_NAME);
        fn_process(input, output, config.clone()).await?;
        Ok(())
    }
}

/// Компонент cmp_surrealdb
pub type Cmp<TMsg> = Component<super::Config<TMsg>, TMsg>;
