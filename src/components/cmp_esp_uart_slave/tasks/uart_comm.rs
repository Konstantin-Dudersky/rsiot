use std::time::Duration;

use esp_idf_svc::hal::{
    gpio::{Output, Pin, PinDriver},
    io::asynch::Write,
    uart::{AsyncUartDriver, UartDriver},
};
use tokio::time::sleep;
use tracing::{trace, warn};

use crate::components_config::uart_general::{FieldbusRequest, FieldbusResponse};

use super::super::TFnUartComm;
use super::Buffer;

pub struct UartComm<TBufferData, TPinRts>
where
    TPinRts: Pin,
{
    pub address: u8,
    pub uart: AsyncUartDriver<'static, UartDriver<'static>>,
    pub pin_rts: PinDriver<'static, TPinRts, Output>,
    pub fn_uart_comm: TFnUartComm<FieldbusRequest, FieldbusResponse, TBufferData>,
    pub buffer_data: Buffer<TBufferData>,
}

const BUFFER_LEN: usize = 100;

impl<TBufferData, TPinRts> UartComm<TBufferData, TPinRts>
where
    TPinRts: Pin,
{
    pub async fn spawn<const MESSAGE_LEN: usize>(mut self) -> super::Result<()> {
        loop {
            let mut read_bufffer = [0_u8; BUFFER_LEN];

            let res = self.uart.read(&mut read_bufffer).await;
            if let Err(err) = res {
                warn!("Error reading from uart: {:?}", err);
                continue;
            }

            trace!("Read UART buffer: {:?}", read_bufffer);

            let request = match FieldbusRequest::from_read_buffer(&mut read_bufffer) {
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

            let write_buffer: [u8; MESSAGE_LEN] = response.to_write_buffer()?;

            self.pin_rts.set_high().unwrap();
            self.uart.write_all(&write_buffer).await.unwrap();
            sleep(Duration::from_millis(10)).await;
            self.pin_rts.set_low().unwrap();
        }
    }
}
