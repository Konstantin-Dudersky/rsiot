use esp_idf_svc::hal::{
    io::asynch::Write,
    uart::{AsyncUartDriver, UartDriver},
};
use tracing::{info, trace, warn};

use crate::components_config::uart_general::{FieldbusRequest, FieldbusResponse};

use super::{super::TFnUartComm, Buffer, Error};

pub struct UartComm<TBufferData> {
    pub uart: AsyncUartDriver<'static, UartDriver<'static>>,
    pub fn_uart_comm: TFnUartComm<FieldbusRequest, FieldbusResponse, TBufferData>,
    pub buffer_data: Buffer<TBufferData>,
}

const READ_BUFFER_LEN: usize = 100;

impl<TBufferData> UartComm<TBufferData> {
    pub async fn spawn(mut self) -> super::Result<()> {
        let port = self.uart.driver().port();
        info!("Start uart communication on port: {}", port);

        let mut read_buffer = [0_u8; READ_BUFFER_LEN];

        loop {
            // Очистка буфера. Очищать быстрее, чем создавать массив
            read_buffer.iter_mut().for_each(|x| *x = 0);

            let read_len = self.uart.read(&mut read_buffer).await;

            let read_len = match read_len {
                Ok(val) => val,
                Err(err) => {
                    warn!("Error reading from uart: {:?}", err);
                    continue;
                }
            };

            trace!("Read UART buffer: {:?}", read_buffer);

            let fieldbus_request = FieldbusRequest::from_read_buffer(&read_buffer[..read_len]);

            trace!("Request: {:?}", fieldbus_request);

            let response = {
                let mut buffer_data = self.buffer_data.lock().await;
                (self.fn_uart_comm)(fieldbus_request, &mut buffer_data)
            };

            let response = match response {
                Ok(val) => val,
                Err(err) => {
                    let err = format!("fn_uart_comm error: {err}");
                    warn!("{err}");
                    continue;
                }
            };
            let Some(response) = response else {
                continue;
            };

            trace!("Response: {:?}", response);

            let write_buffer = response.to_write_buffer();

            self.uart
                .write_all(&write_buffer)
                .await
                .map_err(Error::UartWriteAll)?;
            self.uart.flush().await.map_err(Error::UartFlush)?;
        }
    }
}
