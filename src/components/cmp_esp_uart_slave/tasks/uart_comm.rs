use std::time::Duration;

use esp_idf_svc::hal::{
    gpio::{Output, Pin, PinDriver},
    io::asynch::Write,
    uart::{AsyncUartDriver, UartDriver},
};
use log::info;
use tokio::time::sleep;
use tracing::{trace, warn};

use crate::components::cmp_esp_uart_slave::{RequestResponseBound, UartMessage};

use super::super::TFnUartComm;
use super::Buffer;

pub struct UartComm<TRequest, TResponse, TBufferData, TPinRts>
where
    TPinRts: Pin,
{
    pub address: u8,
    pub uart: AsyncUartDriver<'static, UartDriver<'static>>,
    pub pin_rts: PinDriver<'static, TPinRts, Output>,
    pub fn_uart_comm: TFnUartComm<TRequest, TResponse, TBufferData>,
    pub buffer_data: Buffer<TBufferData>,
}

const BUFFER_LEN: usize = 100;

impl<TRequest, TResponse, TBufferData, TPinRts> UartComm<TRequest, TResponse, TBufferData, TPinRts>
where
    TRequest: RequestResponseBound,
    TResponse: RequestResponseBound,
    TPinRts: Pin,
{
    pub async fn spawn<const MESSAGE_LEN: usize>(mut self) -> super::Result<()> {
        loop {
            let mut buf = [0_u8; BUFFER_LEN];

            let res = self.uart.read(&mut buf).await;
            if let Err(err) = res {
                warn!("Error reading from uart: {:?}", err);
                continue;
            }

            trace!("Read UART buffer: {:?}", buf);

            let request = UartMessage::deserialize(&mut buf);
            let request: UartMessage<TRequest> = match request {
                Ok(val) => val,
                Err(_) => continue,
            };
            trace!("Request: {:?}", request);

            if request.address != self.address {
                continue;
            }

            let response = {
                let mut buffer_data = self.buffer_data.lock().await;
                (self.fn_uart_comm)(request.payload, &mut buffer_data)
            };

            let response = match response {
                Ok(val) => val,
                Err(err) => {
                    let err = format!("fn_uart_comm error: {err}");
                    warn!("{err}");
                    continue;
                }
            };

            let response = UartMessage {
                address: self.address,
                payload: response,
            };
            trace!("Response: {:?}", response);

            let response: [u8; MESSAGE_LEN] = response.serialize().unwrap();

            self.pin_rts.set_high().unwrap();
            self.uart.write_all(&response).await.unwrap();
            sleep(Duration::from_millis(10)).await;
            self.pin_rts.set_low().unwrap();
        }
    }
}
