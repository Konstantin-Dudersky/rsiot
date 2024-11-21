use std::{sync::Arc, thread::sleep, time::Duration};

use linux_embedded_hal::gpio_cdev::LineHandle;
use serialport::SerialPort;
use tokio::sync::{mpsc, Mutex};
use tracing::trace;

use crate::executor::CheckCapacity;

use super::super::UartMessageRaw;

pub struct UartWrite {
    pub input: mpsc::Receiver<UartMessageRaw>,
    pub port: Arc<Mutex<Box<dyn SerialPort>>>,
    pub wait_after_write: Duration,
    pub pin_rts: LineHandle,
}

impl UartWrite {
    pub fn spawn(mut self) -> super::Result<()> {
        while let Some(msg) = self.input.blocking_recv() {
            self.input.check_capacity(0.2, "uart_write");

            trace!("Send: {:?}", msg);
            {
                let mut port = self.port.blocking_lock();

                self.pin_rts.set_value(1).unwrap();
                port.write_all(&msg).expect("Write failed!");

                // Задержка перед сбросом пина RTS
                sleep(Duration::from_millis(1));
                self.pin_rts.set_value(0).unwrap();

                // Задержка перед следующими запросами. Ожидание ответа от подчиненных устройств
                sleep(self.wait_after_write);
            }
        }

        Err(super::Error::TaskEndUartComm)
    }
}
