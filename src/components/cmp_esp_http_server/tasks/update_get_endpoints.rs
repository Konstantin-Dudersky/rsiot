use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{
    components_config::http_server::GetEndpointsHashMap,
    executor::CmpInOut,
    message::{MsgDataBound, ServiceBound},
};

pub struct UpdateGetEndpoints<TMsg, TService>
where
    TMsg: MsgDataBound,
    TService: ServiceBound,
{
    pub input: CmpInOut<TMsg, TService>,
    pub get_endpoints: Arc<Mutex<GetEndpointsHashMap<TMsg>>>,
}

impl<TMsg, TService> UpdateGetEndpoints<TMsg, TService>
where
    TMsg: MsgDataBound,
    TService: ServiceBound,
{
    pub async fn spawn(mut self) -> super::Result<()> {
        while let Ok(msg) = self.input.recv_input().await {
            let mut get_endpoints = self.get_endpoints.lock().await;

            for endpoint in get_endpoints.values_mut() {
                endpoint.fn_input(&msg);
            }
        }

        Err(super::Error::TaskEndUpdateGetEndpoints)
    }
}
