use std::{sync::Arc, thread::sleep, time::Duration};

use linux_embedded_hal::gpio_cdev::LineHandle;
use serialport::SerialPort;
use tokio::sync::{mpsc, Mutex};
use tracing::trace;

use crate::executor::CheckCapacity;

use super::UartMessageRaw;

pub struct UartWrite<const MESSAGE_LEN: usize> {
    pub input: mpsc::Receiver<UartMessageRaw<MESSAGE_LEN>>,
    pub port: Arc<Mutex<Box<dyn SerialPort>>>,
    pub wait_after_write: Duration,
    pub pin_rts: Option<LineHandle>,
}

impl<const MESSAGE_LEN: usize> UartWrite<MESSAGE_LEN> {
    pub fn spawn(mut self) -> super::Result<()> {
        while let Some(msg) = self.input.blocking_recv() {
            self.input.check_capacity(0.2, "uart_write");

            trace!("Send: {:?}", msg);
            {
                let mut port = self.port.blocking_lock();

                if let Some(pin_rts) = &self.pin_rts {
                    pin_rts
                        .set_value(1)
                        .map_err(|e| super::Error::GpioPinSet(e.to_string()))?;
                }
                port.write_all(&msg)
                    .map_err(|e| super::Error::UartWrite(e.to_string()))?;

                // Задержка перед сбросом пина RTS
                sleep(Duration::from_millis(1));
                if let Some(pin_rts) = &self.pin_rts {
                    pin_rts
                        .set_value(0)
                        .map_err(|e| super::Error::GpioPinSet(e.to_string()))?;
                }

                // Задержка перед следующими запросами. Ожидание ответа от подчиненных устройств
                sleep(self.wait_after_write);
            }
        }

        Err(super::Error::TaskEndUartComm)
    }
}
