use std::{io, thread::sleep, time::Duration};

use linux_embedded_hal::serialport;
use tracing::warn;

use crate::{
    components_config::uart_general::{Baudrate, DataBits, Parity, StopBits},
    executor::MsgBusOutput,
    message::MsgDataBound,
};

use super::{COMPONENT_NAME, Error};

const READ_BUFFER_LEN: usize = 1000;
const READ_BUFFER_CHUNK: usize = 32;

pub struct TaskSerialComm<TMsg>
where
    TMsg: MsgDataBound,
{
    pub output: MsgBusOutput<TMsg>,
    pub port: &'static str,
    pub baudrate: Baudrate,
    pub data_bits: DataBits,
    pub parity: Parity,
    pub stop_bits: StopBits,
    pub timeout: Duration,
    pub fn_output: fn(&[u8]) -> Result<Option<TMsg>, anyhow::Error>,
}

impl<TMsg> TaskSerialComm<TMsg>
where
    TMsg: MsgDataBound,
{
    pub fn spawn(&self) -> Result<(), Error> {
        loop {
            let res = self.loop_();
            warn!("{:?}", res);
            sleep(Duration::from_secs(2));
        }
    }

    fn loop_(&self) -> Result<(), Error> {
        let serial_port_builder = serialport::new("", 0)
            .path(self.port)
            .baud_rate(self.baudrate.into())
            .data_bits(self.data_bits.into())
            .parity(self.parity.into())
            .stop_bits(self.stop_bits.into())
            .timeout(self.timeout);
        let mut port = serial_port_builder.open().map_err(Error::PortOpen)?;

        port.clear(serialport::ClearBuffer::All)
            .map_err(Error::PortClear)?;

        loop {
            let mut read_buffer = vec![0; READ_BUFFER_LEN];
            let mut read_buffer_offset: usize = 0;

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
                            break Ok(None);
                        } else {
                            break Ok(Some(read_buffer[..read_buffer_offset].to_vec()));
                        }
                    }
                    Err(e) => break Err(Error::PortRead(e.to_string())),
                }
            };

            let packet = match read_buffer {
                Ok(Some(packet)) => packet,
                Ok(None) => continue,
                Err(err) => {
                    let err = err.to_string();
                    let err = format!("UART read error: {}", err);
                    return Err(Error::PortRead(err));
                }
            };

            let msg = (self.fn_output)(&packet);

            let msg = match msg {
                Ok(Some(msg)) => msg,
                Ok(None) => continue,
                Err(err) => {
                    warn!("Failed to process packet: {:?}", err);
                    continue;
                }
            };

            let msg = msg.to_message();

            let res = self.output.try_send(msg);

            if res.is_err() {
                warn!("Bus in component {COMPONENT_NAME} is full");
            }
        }
    }
}
