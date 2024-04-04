use crate::executor::CmpInOut;

use super::Config;

#[derive(Clone)]
pub struct SharedState<TMsg>
where
    TMsg: Clone,
{
    pub cmp_interface: CmpInOut<TMsg>,
    pub config: Config<TMsg>,
}
