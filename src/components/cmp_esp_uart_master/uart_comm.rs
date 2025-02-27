use std::{thread::sleep, time::Duration};

use esp_idf_svc::hal::uart::{AsyncUartDriver, UartDriver};
use tokio::sync::{broadcast, mpsc};
use tracing::{trace, warn};

use crate::{
    components_config::uart_general::{self, calculate_transmission_time, UartResponse},
    executor::CheckCapacity,
};

use super::bytes_per_second;

const READ_BUFFER_LEN: usize = 100;
const READ_BUFFER_CHUNK: usize = 32;

pub struct UartComm {
    pub ch_rx_devices_to_fieldbus: mpsc::Receiver<uart_general::UartRequest>,
    pub ch_tx_fieldbus_to_devices: broadcast::Sender<uart_general::UartResponse>,

    pub uart_driver: AsyncUartDriver<'static, UartDriver<'static>>,

    pub timeout: Duration,
}

impl UartComm {
    pub async fn spawn(mut self) -> super::Result<()> {
        while let Some(request) = self.ch_rx_devices_to_fieldbus.recv().await {
            self.ch_rx_devices_to_fieldbus
                .check_capacity(0.2, "uart_write");
            let address = request.address;
            let request_creation_time = request.request_creation_time;

            trace!("Send: {:?}", request);

            let write_buffer = request.to_write_buffer()?;

            // Записываем буфер и ждем, пока данные отправятся
            self.uart_driver
                .write(&write_buffer)
                .await
                .map_err(|e| super::Error::UartWrite(e.to_string()))?;

            let mut read_buffer = vec![0; READ_BUFFER_LEN];
            let mut read_buffer_offset: usize = 0;

            let port_read_result = self.uart_driver.read(&mut read_buffer).await;

            // Читаем данные из порта по частям
            let read_buffer = loop {
                let mut read_buffer_chunk = vec![0; READ_BUFFER_CHUNK];
                let port_read_result = port.read(&mut read_buffer_chunk);
                match port_read_result {
                    Ok(bytes_read) => {
                        // Перемещаем все данные в один буфер
                        (0..bytes_read).for_each(|i| {
                            read_buffer[i + read_buffer_offset] = read_buffer_chunk[i];
                        });
                        // Увеличиваем смещение на количество прочитанных байт
                        read_buffer_offset += bytes_read;
                    }
                    Err(err) => break Err(err),
                }
                // Пробуем востановить ответ. В ответе содержится CRC32. Если контрольная сумма
                // совпала - прекращаем читать из буфера, возвращаем ответ. Если не совпала,
                // то опять читаем из буфера
                let response = UartResponse::from_read_buffer(&mut read_buffer);
                if let Ok(read_buffer) = response {
                    break Ok(read_buffer);
                }
            };

            let mut response = match read_buffer {
                Ok(val) => val,
                Err(err) => {
                    let err = err.to_string();
                    warn!("UART read error: {}; address: {}", err, address);
                    // TODO - возможно, отправлять на устройство ответ, что есть проблема чтения
                    continue;
                }
            };

            response.set_request_creation_time(request_creation_time);

            self.ch_tx_fieldbus_to_devices
                .send(response)
                .map_err(|e| super::Error::TokioSyncBroadcastSend(e.to_string()))?;
        }

        Ok(())
    }
}
