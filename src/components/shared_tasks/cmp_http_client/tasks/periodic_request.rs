use std::time::Duration;

use tokio::{
    sync::{broadcast, mpsc},
    time::sleep,
};

use crate::{
    components_config::http_client::{MsgRequest, RequestPeriodic},
    message::{Message, MsgDataBound},
};

use super::{Error, Result};

pub struct PeriodicRequest<TMsg>
where
    TMsg: MsgDataBound,
{
    pub output: mpsc::Sender<MsgRequest>,
    pub request_periodic: Box<dyn RequestPeriodic<TMsg>>,
}

impl<TMsg> PeriodicRequest<TMsg>
where
    TMsg: MsgDataBound,
{
    pub async fn spawn(self) -> Result<()> {
        loop {
            let request = self.request_periodic.create_request();

            if let Some(request) = request {
                self.output
                    .send(request)
                    .await
                    .map_err(|_| Error::TokioSyncMpscSend)?;
            }

            sleep(self.request_periodic.get_period()).await;
        }
    }
}
