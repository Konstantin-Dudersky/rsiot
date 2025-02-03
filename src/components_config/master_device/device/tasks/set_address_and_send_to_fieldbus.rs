use tokio::sync::mpsc;
use tracing::trace;

use super::{AddressBound, RequestResponseBound};

pub async fn set_address_and_send_to_fieldbus<TFieldbusRequest, TAddress>(
    mut requests: Vec<TFieldbusRequest>,
    address: TAddress,
    ch_tx_device_to_fieldbus: &mpsc::Sender<TFieldbusRequest>,
) where
    TFieldbusRequest: RequestResponseBound<TAddress>,
    TAddress: AddressBound,
{
    for request in requests.iter_mut() {
        request.set_address(address);
    }

    for request in requests {
        trace!("Request: {:?}", request);
        ch_tx_device_to_fieldbus.send(request).await.unwrap();
    }
}
