use std::sync::Arc;

use esp_idf_hal::{
    io::asynch::Write,
    uart::{AsyncUartDriver, UartDriver},
};
use tokio::sync::Mutex;
use tracing::{info, warn};

use crate::components::cmp_esp_uart_slave::{RequestResponseBound, UartMessage};

use super::super::TFnUartComm;

pub struct UartComm<TRequest, TResponse, TBufferData> {
    pub address: u8,
    pub uart: AsyncUartDriver<'static, UartDriver<'static>>,
    pub fn_uart_comm: TFnUartComm<TRequest, TResponse, TBufferData>,
    pub buffer_data: Arc<Mutex<TBufferData>>,
}

const BUFFER_LEN: usize = 100;

impl<TRequest, TResponse, TBufferData> UartComm<TRequest, TResponse, TBufferData>
where
    TRequest: RequestResponseBound,
    TResponse: RequestResponseBound,
{
    pub async fn spawn(mut self) -> super::Result<()> {
        loop {
            let mut buf = [0_u8; BUFFER_LEN];

            let res = self.uart.read(&mut buf).await;
            if let Err(err) = res {
                warn!("Error reading from uart: {:?}", err);
                continue;
            }

            let request = UartMessage::deserialize(&mut buf);
            let request: UartMessage<TRequest> = match request {
                Ok(val) => val,
                Err(_) => continue,
            };
            info!("Request: {:?}", request);

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
            info!("Response: {:?}", response);

            let response = response.serialize().unwrap();

            self.uart.write_all(&response).await.unwrap();
        }
    }
}
