use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{executor::MsgBusOutput, message::MsgDataBound};

use super::{GetEndpointsCollection, PutEndpointsCollection};

#[derive(Clone)]
pub struct SharedState<TMsg>
where
    TMsg: MsgDataBound,
{
    pub msgbus_output: MsgBusOutput<TMsg>,
    pub get_endpoints: Arc<Mutex<GetEndpointsCollection<TMsg>>>,
    pub put_endpoints: Arc<Mutex<PutEndpointsCollection<TMsg>>>,
}
