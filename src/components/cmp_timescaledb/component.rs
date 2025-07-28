use async_trait::async_trait;

use crate::message::{AuthPermissions, MsgDataBound};

use crate::executor::{CmpInOut, Component, ComponentError, IComponentProcess};

use super::{config::Config, fn_process::fn_process};

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMsg> IComponentProcess<Config<TMsg>, TMsg> for Component<Config<TMsg>, TMsg>
where
    TMsg: 'static + MsgDataBound,
{
    async fn process(
        &self,
        config: Config<TMsg>,
        input: CmpInOut<TMsg>,
    ) -> Result<(), ComponentError> {
        fn_process(
            input.clone_with_new_id("cmp_timescaledb_storing", AuthPermissions::FullAccess),
            config,
        )
        .await
    }
}

/// Компонент cmp_timescaledb
pub type Cmp<TMsg> = Component<Config<TMsg>, TMsg>;
