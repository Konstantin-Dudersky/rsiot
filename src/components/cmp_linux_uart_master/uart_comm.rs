use std::time::Duration;

use linux_embedded_hal::gpio_cdev::{Chip, LineRequestFlags};
use serde::{Deserialize, Serialize};
use tokio::{
    sync::{broadcast, mpsc},
    time::sleep,
};
use tracing::trace;

use crate::{
    components_config::uart_general::{self, FieldbusResponse},
    serde_utils::postcard_serde::{self, MESSAGE_LEN},
};

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
    pub async fn spawn(mut self) -> super::Result<()> {
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

            trace!("Send: {:?}", request);

            let uart_message = UartMessage {
                address: request.address,
                payload: request.uart_request,
            };
            let uart_message: [u8; MESSAGE_LEN] =
                postcard_serde::serialize_crc_arr(&uart_message).unwrap();

            if let Some(pin_rts) = &pin_rts {
                pin_rts
                    .set_value(1)
                    .map_err(|e| super::Error::GpioPinSet(e.to_string()))?;
            }
            port.write_all(&uart_message)
                .map_err(|e| super::Error::UartWrite(e.to_string()))?;

            // Задержка перед сбросом пина RTS
            sleep(Duration::from_millis(1)).await;
            if let Some(pin_rts) = &pin_rts {
                pin_rts
                    .set_value(0)
                    .map_err(|e| super::Error::GpioPinSet(e.to_string()))?;
            }

            // Чтение ------------------------------------------------------------------------------

            let mut read_buf = [0; MESSAGE_LEN];

            let port_read_result = port.read_exact(&mut read_buf);
            if port_read_result.is_err() {
                continue;
            }

            let response: UartMessage = postcard_serde::deserialize_crc(&mut read_buf).unwrap();
            let response = FieldbusResponse {
                address: response.address,
                request_creation_time: request.request_creation_time,
                uart_response: response.payload,
            };

            self.ch_tx_fieldbus_to_devices
                .send(response)
                .map_err(|e| super::Error::TokioSyncBroadcastSend(e.to_string()))?;

            // Задержка перед следующими запросами. Ожидание ответа от подчиненных устройств
            sleep(self.wait_after_write).await;
        }

        Ok(())
    }
}

#[derive(Deserialize, Serialize)]
struct UartMessage {
    pub address: u8,
    pub payload: Vec<u8>,
}
