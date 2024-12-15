use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{
    executor::CmpInOut,
    message::{MsgDataBound, ServiceBound},
};

use super::Config;

#[derive(Clone)]
pub struct SharedState<TMsg, TService>
where
    TMsg: MsgDataBound,
    TService: ServiceBound,
{
    pub msg_bus: CmpInOut<TMsg, TService>,
    pub config: Config<TMsg>,
    pub cmp_plc_input: String,
    pub cmp_plc_output: String,
    pub cmp_plc_static: String,
}

pub type TSharedState<TMsg, TService> = Arc<Mutex<SharedState<TMsg, TService>>>;
