use std::time::Duration;

use tokio::time::sleep;
use tracing::debug;

use super::Error;

pub struct InterfaceInfo {
    pub ifname: String,
    pub period: Duration,
}

impl InterfaceInfo {
    pub async fn spawn(self) -> Result<(), Error> {
        let interface = socketcan::CanInterface::open(&self.ifname)
            .map_err(|e| Error::InterfaceOpen(e.to_string()))?;

        loop {
            let state = interface
                .state()
                .map_err(|e| Error::InterfaceState(e.to_string()))?;
            debug!("State: {:?}", state);

            // let berr_counter = interface
            //     .berr_counter()
            //     .map_err(|e| Error::InterfaceState(e.to_string()))?;
            // debug!("Err counter: {:?}", berr_counter);

            sleep(self.period).await;
        }
    }
}
