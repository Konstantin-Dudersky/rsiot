use std::fmt::Debug;

use async_trait::async_trait;
use serde::Serialize;

use rsiot_component_core::{
    cmp_set_component_id, Cache, CmpInput, CmpOutput, Component, ComponentError, IComponentProcess,
};

use super::{fn_process::fn_process, Config};

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMsg> IComponentProcess<Config<TMsg>, TMsg> for Component<Config<TMsg>, TMsg>
where
    TMsg: Clone + Debug + Send + Serialize + Sync + 'static,
{
    async fn process(
        &self,
        config: Config<TMsg>,
        mut input: CmpInput<TMsg>,
        mut output: CmpOutput<TMsg>,
        _cache: Cache<TMsg>,
    ) -> Result<(), ComponentError> {
        cmp_set_component_id(&mut input, &mut output, "cmp_derive");
        fn_process(input, output, config)
            .await
            .map_err(|e| ComponentError::Execution(e.to_string()))
    }
}

pub type Cmp<TMsg> = Component<Config<TMsg>, TMsg>;
