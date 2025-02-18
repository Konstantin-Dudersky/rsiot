use esp_idf_svc::{
    hal::{
        io::asynch::Write,
        uart::{AsyncUartDriver, UartDriver},
    },
    sys::uart_flush_input,
};
use tracing::{trace, warn};

use crate::components_config::uart_general::{UartRequest, UartResponse};

use super::super::TFnUartComm;
use super::Buffer;

pub struct UartComm<TBufferData> {
    pub address: u8,
    pub uart: AsyncUartDriver<'static, UartDriver<'static>>,
    pub fn_uart_comm: TFnUartComm<UartRequest, UartResponse, TBufferData>,
    pub buffer_data: Buffer<TBufferData>,
}

const READ_BUFFER_LEN: usize = 100;

impl<TBufferData> UartComm<TBufferData> {
    pub async fn spawn(mut self) -> super::Result<()> {
        let port = self.uart.driver().port();

        loop {
            let mut read_buffer = [0_u8; READ_BUFFER_LEN];

            // Очистка буфера чтения
            // Используется unsafe функция, поскольку AsyncUartDriver не содержит метода clear_rx()
            unsafe { uart_flush_input(port) };

            let res = self.uart.read(&mut read_buffer).await;
            let _read_len = match res {
                Ok(val) => val,
                Err(err) => {
                    warn!("Error reading from uart: {:?}", err);
                    continue;
                }
            };

            trace!("Read UART buffer: {:?}", read_buffer);

            let request = match UartRequest::from_read_buffer(&mut read_buffer) {
                Ok(val) => val,
                Err(err) => {
                    warn!("Deserialization error: {:?}", err);
                    continue;
                }
            };

            if request.address != self.address {
                continue;
            }

            trace!("Request: {:?}", request);
            let address = request.address;

            let response = {
                let mut buffer_data = self.buffer_data.lock().await;
                (self.fn_uart_comm)(request, &mut buffer_data)
            };

            let mut response = match response {
                Ok(val) => val,
                Err(err) => {
                    let err = format!("fn_uart_comm error: {err}");
                    warn!("{err}");
                    continue;
                }
            };
            response.address = address;

            trace!("Response: {:?}", response);

            let write_buffer = response.to_write_buffer()?;

            self.uart.write_all(&write_buffer).await.unwrap();
            self.uart.flush().await.unwrap();
        }
    }
}
