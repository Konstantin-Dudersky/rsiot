use std::{thread::sleep, time::Duration};

use linux_embedded_hal::{
    gpio_cdev::{Chip, LineRequestFlags},
    serialport,
};
use tokio::sync::{broadcast, mpsc};
use tracing::{trace, warn};

use crate::{
    components_config::uart_general::{self, calculate_transmission_time, UartResponse},
    executor::CheckCapacity,
};

use super::data_rate;

const READ_BUFFER_LEN: usize = 100;
const READ_BUFFER_CHUNK: usize = 32;

pub struct UartComm {
    pub ch_rx_devices_to_fieldbus: mpsc::Receiver<uart_general::UartRequest>,
    pub ch_tx_fieldbus_to_devices: broadcast::Sender<uart_general::UartResponse>,
    pub pin_rts: Option<u32>,

    pub timeout: Duration,
    pub port: &'static str,
    pub baudrate: uart_general::Baudrate,
    pub data_bits: uart_general::DataBits,
    pub parity: uart_general::Parity,
    pub stop_bits: uart_general::StopBits,
    pub gpio_chip: &'static str,
}

impl UartComm {
    pub fn spawn(mut self) -> super::Result<()> {
        let data_rate = data_rate(
            &self.baudrate,
            &self.data_bits,
            &self.parity,
            &self.stop_bits,
        );

        let serial_port_builder = serialport::new("", 0)
            .path(self.port)
            .baud_rate(self.baudrate.into())
            .data_bits(self.data_bits.into())
            .parity(self.parity.into())
            .stop_bits(self.stop_bits.into())
            .timeout(self.timeout);
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

        while let Some(request) = self.ch_rx_devices_to_fieldbus.blocking_recv() {
            // TODO
            self.ch_rx_devices_to_fieldbus
                .check_capacity(0.2, "uart_write");
            let address = request.address;
            let request_creation_time = request.request_creation_time;

            trace!("Send: {:?}", request);

            let write_buffer = request.to_write_buffer()?;

            // Устанавливаем пин RTS
            if let Some(pin_rts) = &pin_rts {
                pin_rts
                    .set_value(1)
                    .map_err(|e| super::Error::GpioPinSet(e.to_string()))?;
            }
            port.clear(serialport::ClearBuffer::All).unwrap();

            // Записываем буфер и ждем, пока данные отправятся
            port.write(&write_buffer)
                .map_err(|e| super::Error::UartWrite(e.to_string()))?;

            // Рассчитываем время передачи данных.
            // Если использовать port.flush(), то время ожидания будет больше примерно на 10 мс
            let transmission_time = calculate_transmission_time(
                data_rate,
                write_buffer.len(),
                Duration::from_millis(0),
            );
            sleep(transmission_time);

            port.clear(serialport::ClearBuffer::All).unwrap();

            // Сбрасываем пин RTS
            if let Some(pin_rts) = &pin_rts {
                pin_rts
                    .set_value(0)
                    .map_err(|e| super::Error::GpioPinSet(e.to_string()))?;
            }

            let mut read_buffer = vec![0; READ_BUFFER_LEN];
            let mut read_buffer_offset: usize = 0;

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
