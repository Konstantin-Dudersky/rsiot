use std::time::Duration;

use tokio::{sync::mpsc, time::sleep};

use super::{
    set_address_and_send_to_fieldbus::set_address_and_send_to_fieldbus, Buffer,
    RequestResponseBound,
};

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
            let requests = {
                let mut buffer = self.buffer.lock().await;
                (self.fn_request)(&mut buffer)
            };

            set_address_and_send_to_fieldbus(
                requests,
                self.address,
                &self.ch_tx_device_to_fieldbus,
            )
            .await;

            sleep(self.period).await
        }
    }
}
