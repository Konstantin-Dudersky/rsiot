use std::sync::Arc;

use serialport::SerialPort;
use tokio::sync::{mpsc, Mutex};
use tracing::trace;

use super::super::UartMessageRaw;

pub struct UartWrite {
    pub input: mpsc::Receiver<UartMessageRaw>,
    pub port: Arc<Mutex<Box<dyn SerialPort>>>,
}

impl UartWrite {
    pub fn spawn(mut self) -> super::Result<()> {
        while let Some(msg) = self.input.blocking_recv() {
            trace!("Send: {:?}", msg);
            {
                let mut port = self.port.blocking_lock();
                port.write_all(&msg).expect("Write failed!");
            }
        }

        Err(super::Error::TaskEndUartComm)
    }
}
