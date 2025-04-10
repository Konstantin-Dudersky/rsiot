use tokio::sync::mpsc;
use tracing::trace;

use super::{Error, RequestResponseBound};

pub struct InitRequest<TFieldbusRequest> {
    pub fn_init_requests: fn() -> Vec<TFieldbusRequest>,
    pub ch_tx_device_to_fieldbus: mpsc::Sender<TFieldbusRequest>,
}

impl<TFieldbusRequest> InitRequest<TFieldbusRequest>
where
    TFieldbusRequest: RequestResponseBound,
{
    pub async fn spawn(self) -> super::Result<()> {
        let requests = (self.fn_init_requests)();

        for request in requests {
            trace!("Request: {:?}", request);
            self.ch_tx_device_to_fieldbus
                .send(request)
                .await
                .map_err(|_| Error::TokioSyncMpsc)?;
        }

        Ok(())
    }
}
