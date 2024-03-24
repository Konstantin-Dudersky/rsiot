use async_trait::async_trait;

use crate::message::{AuthPermissions, MsgDataBound};

use crate::executor::{CmpInOut, Component, ComponentError, IComponentProcess};

use super::{config::ConfigAlias, fn_process::fn_process};

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMsg> IComponentProcess<ConfigAlias, TMsg> for Component<ConfigAlias, TMsg>
where
    TMsg: MsgDataBound,
{
    async fn process(
        &self,
        config: ConfigAlias,
        input: CmpInOut<TMsg>,
    ) -> Result<(), ComponentError> {
        let config = config.0;
        fn_process(
            input.clone_with_new_id("cmp_timescaledb_storing", AuthPermissions::FullAccess),
            config,
        )
        .await
    }
}

/// Компонент cmp_timescaledb
pub type Cmp<TMsg> = Component<ConfigAlias, TMsg>;
