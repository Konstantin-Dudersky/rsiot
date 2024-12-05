use std::{sync::Arc, thread::sleep, time::Duration};

use serialport::SerialPort;
use tokio::sync::{broadcast, Mutex};
use tracing::trace;

use super::UartMessageRaw;

pub struct UartRead<const MESSAGE_LEN: usize> {
    pub output: broadcast::Sender<UartMessageRaw<MESSAGE_LEN>>,
    pub port: Arc<Mutex<Box<dyn SerialPort>>>,
}

impl<const MESSAGE_LEN: usize> UartRead<MESSAGE_LEN> {
    pub fn spawn(self) -> super::Result<()> {
        loop {
            let mut port = self.port.blocking_lock();

            let mut read_buf = [0; MESSAGE_LEN];

            let port_read_result = port.read_exact(&mut read_buf);
            if port_read_result.is_err() {
                sleep(Duration::from_millis(10));
                continue;
            }

            trace!("Read: {:?}", read_buf);

            self.output
                .send(read_buf)
                .map_err(|e| super::Error::TokioSyncBroadcastSend(e.to_string()))?;
        }
    }
}
