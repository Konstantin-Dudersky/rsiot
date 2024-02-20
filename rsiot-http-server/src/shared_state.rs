use crate::Config;
use rsiot_component_core::{Cache, CmpOutput};

#[derive(Clone)]
pub struct SharedState<TMsg>
where
    TMsg: Clone,
{
    pub output: CmpOutput<TMsg>,
    pub cache: Cache<TMsg>,
    pub config: Config<TMsg>,
}
