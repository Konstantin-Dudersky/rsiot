use std::time::Duration;

use linux_embedded_hal::gpio_cdev::{Chip, LineRequestFlags};
use tokio::{
    sync::{broadcast, mpsc},
    time::sleep,
};
use tracing::{info, trace, warn};

use crate::components_config::uart_general::{self, FieldbusResponse};

pub struct UartComm {
    pub ch_rx_devices_to_fieldbus: mpsc::Receiver<uart_general::FieldbusRequest>,
    pub ch_tx_fieldbus_to_devices: broadcast::Sender<uart_general::FieldbusResponse>,
    pub pin_rts: Option<u32>,

    pub wait_after_write: Duration,
    pub port: &'static str,
    pub baudrate: uart_general::Baudrate,
    pub data_bits: uart_general::DataBits,
    pub parity: uart_general::Parity,
    pub stop_bits: uart_general::StopBits,
    pub gpio_chip: &'static str,
}

impl UartComm {
    pub async fn spawn<const MESSAGE_LEN: usize>(mut self) -> super::Result<()> {
        let serial_port_builder = serialport::new("", 0)
            .path(self.port)
            .baud_rate(self.baudrate.into())
            .data_bits(self.data_bits.into())
            .parity(self.parity.into())
            .stop_bits(self.stop_bits.into())
            .timeout(Duration::from_millis(100));
        let mut port = serial_port_builder
            .open()
            .map_err(|e| super::Error::OpenSerialPort(e.to_string()))?;

        // Настраиваем пин для сигнала RTS
        let pin_rts = match self.pin_rts {
            Some(pin_rts) => {
                let mut chip = Chip::new(self.gpio_chip)
                    .map_err(|e| super::Error::GpioSetup(e.to_string()))?;
                let pin_rts = chip
                    .get_line(pin_rts)
                    .map_err(|e| super::Error::GpioSetup(e.to_string()))?;
                let pin_rts = pin_rts
                    .request(LineRequestFlags::OUTPUT, 0, "uart-rts")
                    .map_err(|e| super::Error::GpioSetup(e.to_string()))?;
                Some(pin_rts)
            }
            None => None,
        };

        while let Some(request) = self.ch_rx_devices_to_fieldbus.recv().await {
            // TODO
            // self.ch_rx_devices_to_fieldbus
            //     .check_capacity(0.2, "uart_write");
            let request_creation_time = request.request_creation_time;

            trace!("Send: {:?}", request);

            let write_buffer: [u8; MESSAGE_LEN] = request.to_write_buffer()?;

            if let Some(pin_rts) = &pin_rts {
                pin_rts
                    .set_value(1)
                    .map_err(|e| super::Error::GpioPinSet(e.to_string()))?;
            }
            port.write_all(&write_buffer)
                .map_err(|e| super::Error::UartWrite(e.to_string()))?;

            // Задержка перед сбросом пина RTS
            sleep(Duration::from_millis(1)).await;
            if let Some(pin_rts) = &pin_rts {
                pin_rts
                    .set_value(0)
                    .map_err(|e| super::Error::GpioPinSet(e.to_string()))?;
            }

            // Чтение ------------------------------------------------------------------------------

            // Задержка перед следующими запросами. Ожидание ответа от подчиненных устройств
            sleep(self.wait_after_write).await;

            let mut read_buf = [0; MESSAGE_LEN];

            let port_read_result = port.read_exact(&mut read_buf);
            if port_read_result.is_err() {
                warn!("read error");
                sleep(Duration::from_millis(10)).await;
                continue;
            } else {
                info!("Read: {:?}", read_buf);
            }

            let mut response = FieldbusResponse::from_read_buffer(&mut read_buf).unwrap();
            response.set_request_creation_time(request_creation_time);

            self.ch_tx_fieldbus_to_devices
                .send(response)
                .map_err(|e| super::Error::TokioSyncBroadcastSend(e.to_string()))?;
        }

        Ok(())
    }
}
