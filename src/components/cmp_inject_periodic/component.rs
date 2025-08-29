use async_trait::async_trait;

use crate::{
    executor::{CmpInOut, Component, ComponentError, IComponentProcess},
    message::{AuthPermissions, MsgDataBound},
};

use super::{config::Config, fn_process::fn_process};

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMsg, TFnPeriodic> IComponentProcess<Config<TMsg, TFnPeriodic>, TMsg>
    for Component<Config<TMsg, TFnPeriodic>, TMsg>
where
    TMsg: 'static + MsgDataBound,
    TFnPeriodic: 'static + FnMut() -> Vec<TMsg> + Send + Sync,
{
    async fn process(
        &self,
        config: Config<TMsg, TFnPeriodic>,
        in_out: CmpInOut<TMsg>,
    ) -> Result<(), ComponentError> {
        fn_process(
            config,
            in_out.clone_with_new_id("cmp_inject_periodic", AuthPermissions::FullAccess),
        )
        .await?;
        Ok(())
    }
}

/// Компонент cmp_inject_periodic
pub type Cmp<TMessage, TFnPeriodic> = Component<Config<TMessage, TFnPeriodic>, TMessage>;
