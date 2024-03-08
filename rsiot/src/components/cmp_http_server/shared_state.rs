use super::Config;
use rsiot_component_core::CmpInOut;

#[derive(Clone)]
pub struct SharedState<TMsg>
where
    TMsg: Clone,
{
    pub cmp_interface: CmpInOut<TMsg>,
    pub config: Config<TMsg>,
}
