use crate::Config;
use rsiot_component_core::CmpInOut;

#[derive(Clone)]
pub struct SharedState<TMsg>
where
    TMsg: Clone,
{
    pub output: CmpInOut<TMsg>,
    pub config: Config<TMsg>,
}
