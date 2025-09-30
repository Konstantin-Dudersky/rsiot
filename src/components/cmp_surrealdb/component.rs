use async_trait::async_trait;

use crate::{
    executor::{Component, ComponentError, IComponentProcess, MsgBusLinker},
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
        msgbus_linker: MsgBusLinker<TMsg>,
    ) -> Result<(), ComponentError> {
        fn_process(msgbus_linker.init(COMPONENT_NAME), config.clone()).await?;
        Ok(())
    }
}

/// Компонент cmp_surrealdb
pub type Cmp<TMsg> = Component<super::Config<TMsg>, TMsg>;
