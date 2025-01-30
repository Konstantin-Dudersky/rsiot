use tokio::sync::mpsc;

use super::{
    set_address_and_send_to_fieldbus::set_address_and_send_to_fieldbus, RequestResponseBound,
};

pub struct InitRequest<TFieldbusRequest> {
    pub address: u8,
    pub fn_init_requests: fn() -> Vec<TFieldbusRequest>,
    pub ch_tx_device_to_fieldbus: mpsc::Sender<TFieldbusRequest>,
}

impl<TFieldbusRequest> InitRequest<TFieldbusRequest>
where
    TFieldbusRequest: RequestResponseBound,
{
    pub async fn spawn(self) -> super::Result<()> {
        let requests = (self.fn_init_requests)();

        set_address_and_send_to_fieldbus(requests, self.address, &self.ch_tx_device_to_fieldbus)
            .await;

        Ok(())
    }
}
