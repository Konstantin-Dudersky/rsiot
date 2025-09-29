use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{
    components_config::http_server::GetEndpointsCollection, executor::MsgBusInput,
    message::MsgDataBound,
};

pub struct UpdateGetEndpoints<TMsg>
where
    TMsg: MsgDataBound,
{
    pub input: MsgBusInput<TMsg>,
    pub get_endpoints: Arc<Mutex<GetEndpointsCollection<TMsg>>>,
}

impl<TMsg> UpdateGetEndpoints<TMsg>
where
    TMsg: MsgDataBound,
{
    pub async fn spawn(mut self) -> super::Result<()> {
        while let Ok(msg) = self.input.recv().await {
            let Some(msg) = msg.get_custom_data() else {
                continue;
            };
            let mut get_endpoints = self.get_endpoints.lock().await;
            get_endpoints.fn_input(&msg);
        }

        Err(super::Error::TaskEndUpdateGetEndpoints)
    }
}
