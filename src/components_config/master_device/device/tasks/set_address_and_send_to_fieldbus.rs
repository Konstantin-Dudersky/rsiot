use tokio::sync::mpsc;
use tracing::trace;

use crate::components_config::master_device::RequestResponseBound;

pub async fn set_address_and_send_to_fieldbus<TFieldbusRequest>(
    mut requests: Vec<TFieldbusRequest>,
    address: u8,
    ch_tx_device_to_fieldbus: &mpsc::Sender<TFieldbusRequest>,
) where
    TFieldbusRequest: RequestResponseBound,
{
    for request in requests.iter_mut() {
        request.set_address(address);
    }

    for request in requests {
        trace!("Request: {:?}", request);
        ch_tx_device_to_fieldbus.send(request).await.unwrap();
    }
}
