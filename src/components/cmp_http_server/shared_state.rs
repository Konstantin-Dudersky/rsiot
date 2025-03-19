use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{
    executor::CmpInOut,
    message::{MsgDataBound, ServiceBound},
};

#[derive(Clone)]
pub struct SharedState<TMsg, TService>
where
    TMsg: MsgDataBound,
    TService: ServiceBound,
{
    pub msg_bus: CmpInOut<TMsg, TService>,
    pub get_endpoints: Arc<Mutex<super::GetEndpointsHashMap<TMsg>>>,
    pub put_endpoints: Arc<Mutex<super::PutEndpointsHashMap<TMsg>>>,
}
