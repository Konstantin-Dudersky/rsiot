use tokio::sync::mpsc;
use tracing::trace;

use crate::executor::CheckCapacity;

use super::{Error, RequestResponseBound};

pub struct Request<TRequest> {
    pub ch_rx_request: mpsc::Receiver<TRequest>,
    pub ch_tx_device_to_fieldbus: mpsc::Sender<TRequest>,
}

impl<TRequest> Request<TRequest>
where
    TRequest: RequestResponseBound,
{
    pub async fn spawn(mut self) -> super::Result<()> {
        while let Some(request) = self.ch_rx_request.recv().await {
            trace!("Request: {:?}", request);
            self.ch_tx_device_to_fieldbus
                .check_capacity(0.2, "master_device | Request")
                .send(request)
                .await
                .map_err(|_| Error::TokioSyncMpscSend)?;
        }
        Ok(())
    }
}
