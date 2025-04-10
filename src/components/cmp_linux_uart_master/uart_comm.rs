use std::{io, thread::sleep, time::Duration};

use linux_embedded_hal::{
    gpio_cdev::{Chip, LineRequestFlags},
    serialport,
};
use tokio::sync::mpsc;
use tracing::{trace, warn};

use crate::{
    components_config::{
        master_device::{FieldbusRequestWithIndex, FieldbusResponseWithIndex},
        uart_general::{self, calculate_transmission_time, FieldbusRequest, FieldbusResponse},
    },
    executor::CheckCapacity,
};

use super::{data_rate, Error};

const READ_BUFFER_LEN: usize = 1000;
const READ_BUFFER_CHUNK: usize = 32;

pub struct UartComm {
    pub ch_rx_addindex_to_fieldbus: mpsc::Receiver<FieldbusRequestWithIndex<FieldbusRequest>>,
    pub ch_tx_fieldbus_to_split: mpsc::Sender<FieldbusResponseWithIndex<FieldbusResponse>>,
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
            // Если поставить 0, то могут теряться байты
            .timeout(Duration::from_millis(2));
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

        while let Some(fieldbus_request) = self.ch_rx_addindex_to_fieldbus.blocking_recv() {
            self.ch_rx_addindex_to_fieldbus
                .check_capacity(0.2, "uart_write");
            let device_index = fieldbus_request.device_index;
            let uart_request = fieldbus_request.request;
            let request_creation_time = uart_request.request_creation_time;

            trace!("Send: {:?}", uart_request);

            let write_buffer = uart_request.packet;

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

            sleep(self.timeout);

            // Читаем данные из порта по частям
            let read_buffer = loop {
                let mut read_buffer_chunk = vec![0; READ_BUFFER_CHUNK];
                let port_read_result = port.read(&mut read_buffer_chunk);

                match port_read_result {
                    Ok(bytes_read) => {
                        if read_buffer_offset + bytes_read >= READ_BUFFER_LEN {
                            break Err(Error::BufferFull);
                        }

                        // Перемещаем все данные в один буфер
                        (0..bytes_read).for_each(|i| {
                            read_buffer[i + read_buffer_offset] = read_buffer_chunk[i];
                        });
                        // Увеличиваем смещение на количество прочитанных байт
                        read_buffer_offset += bytes_read;
                    }
                    Err(e) if e.kind() == io::ErrorKind::TimedOut => {
                        // Таймаут также говорит о том, что буфер пуст
                        if read_buffer_offset == 0 {
                            break Err(Error::UartRead(e.to_string()));
                        } else {
                            break Ok(read_buffer[..read_buffer_offset].to_vec());
                        }
                    }
                    Err(e) => break Err(Error::UartRead(e.to_string())),
                }
            };

            let packet = match read_buffer {
                Ok(val) => val,
                Err(err) => {
                    let err = err.to_string();
                    warn!("UART read error: {}; device: {}", err, device_index);
                    // TODO - возможно, отправлять на устройство ответ, что есть проблема чтения
                    continue;
                }
            };

            let fieldbus_response = FieldbusResponse {
                request_creation_time,
                packet,
            };

            let response_with_index = FieldbusResponseWithIndex {
                device_index,
                response: fieldbus_response,
            };
            self.ch_tx_fieldbus_to_split
                .blocking_send(response_with_index)
                .map_err(|_| super::Error::TokioSyncMpscSend)?;
        }

        Ok(())
    }
}
