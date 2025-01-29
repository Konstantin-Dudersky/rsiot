use std::time::Duration;

use tokio::{sync::mpsc, time::sleep};
use tracing::trace;

use super::{Buffer, RequestResponseBound};

pub struct PeriodicRequest<TRequest, TBuffer> {
    pub address: u8,
    pub buffer: Buffer<TBuffer>,
    pub period: Duration,
    pub fn_request: fn(&TBuffer) -> Vec<TRequest>,
    pub ch_tx_device_to_fieldbus: mpsc::Sender<TRequest>,
}

impl<TRequest, TBuffer> PeriodicRequest<TRequest, TBuffer>
where
    TRequest: RequestResponseBound,
{
    pub async fn spawn(self) -> super::Result<()> {
        loop {
            let mut requests = {
                let mut buffer = self.buffer.lock().await;
                (self.fn_request)(&mut buffer)
            };

            // Задаем адрес
            for request in requests.iter_mut() {
                request.set_address(self.address);
            }

            for request in requests {
                trace!("Request: {:?}", request);
                self.ch_tx_device_to_fieldbus.send(request).await.unwrap();
            }

            sleep(self.period).await
        }
    }
}
