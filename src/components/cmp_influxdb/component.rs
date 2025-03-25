use async_trait::async_trait;

use tracing::error;

use crate::{
    executor::{CmpInOut, Component, ComponentError, IComponentProcess},
    message::{AuthPermissions, MsgDataBound},
};

use super::{config::ConfigAlias, fn_process::fn_process};

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMsg> IComponentProcess<ConfigAlias<TMsg>, TMsg> for Component<ConfigAlias<TMsg>, TMsg>
where
    TMsg: MsgDataBound + 'static,
{
    async fn process(
        &self,
        config: ConfigAlias<TMsg>,
        in_out: CmpInOut<TMsg>,
    ) -> Result<(), ComponentError> {
        fn_process(
            in_out.clone_with_new_id("cmp_influxdb", AuthPermissions::FullAccess),
            config.0,
        )
        .await?;
        error!("Influxdb client component end execution");
        Ok(())
    }
}

/// Компонент cmp_influxdb
pub type Cmp<TMsg> = Component<ConfigAlias<TMsg>, TMsg>;
