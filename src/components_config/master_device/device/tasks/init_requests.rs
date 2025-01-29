use tokio::sync::mpsc;
use tracing::trace;

use super::RequestResponseBound;

pub struct InitRequest<TRequest> {
    pub address: u8,
    pub fn_init_requests: fn() -> Vec<TRequest>,
    pub ch_tx_device_to_fieldbus: mpsc::Sender<TRequest>,
}

impl<TRequest> InitRequest<TRequest>
where
    TRequest: RequestResponseBound,
{
    pub async fn spawn(self) -> super::Result<()> {
        let mut requests = (self.fn_init_requests)();

        // Задаем адрес
        for request in requests.iter_mut() {
            request.set_address(self.address);
        }

        for request in requests {
            trace!("Request: {:?}", request);
            self.ch_tx_device_to_fieldbus.send(request).await.unwrap();
        }

        Ok(())
    }
}
