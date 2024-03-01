use crate::Config;
use rsiot_component_core::{Cache, CmpInOut};

#[derive(Clone)]
pub struct SharedState<TMsg>
where
    TMsg: Clone,
{
    pub output: CmpInOut<TMsg>,
    pub cache: Cache<TMsg>,
    pub config: Config<TMsg>,
}
