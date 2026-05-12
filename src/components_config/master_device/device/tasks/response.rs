use tokio::sync::mpsc;
use tracing::{trace, warn};

use crate::{
    executor::CheckCapacity,
    message::{Message, MsgDataBound},
};

use super::{Buffer, DeviceStateType, Error, RequestResponseBound, ResponseResult};

pub struct Response<TMsg, TResponse, TBuffer> {
    pub buffer: Buffer<TBuffer>,
    pub device_state: DeviceStateType,
    pub ch_rx_fieldbus_to_device: mpsc::Receiver<TResponse>,
    pub ch_tx_output_to_filter: mpsc::Sender<Message<TMsg>>,
    pub ch_tx_need_request: mpsc::Sender<()>,
    pub fn_response_to_buffer: fn(TResponse, &mut TBuffer) -> anyhow::Result<ResponseResult>,
    pub fn_buffer_to_msgs: fn(&mut TBuffer) -> Vec<TMsg>,
}

impl<TMsg, TResponse, TBuffer> Response<TMsg, TResponse, TBuffer>
where
    TResponse: RequestResponseBound,
    TMsg: MsgDataBound,
{
    pub async fn spawn(mut self) -> super::Result<()> {
        while let Some(response) = self.ch_rx_fieldbus_to_device.recv().await {
            trace!("Response: {:?}", response);

            let mut buffer = self.buffer.lock().await;

            let req_res = (self.fn_response_to_buffer)(response, &mut buffer);
            let msgs = (self.fn_buffer_to_msgs)(&mut buffer);

            drop(buffer);

            for msg in msgs {
                let msg = Message::new_custom(msg);
                self.ch_tx_output_to_filter
                    .check_capacity(0.2, "master_device | Response | ch_tx_output_to_filter")
                    .send(msg)
                    .await
                    .map_err(|_| Error::TokioSyncMpscSend)?;
            }

            let mut device_state = self.device_state.lock().await;

            match req_res {
                Ok(req_res) => match req_res {
                    ResponseResult::OkInitCompleted => {
                        device_state.init_completed = true;
                        device_state.response_ok_count += 1
                    }
                    ResponseResult::OkNeedRequest => {
                        device_state.response_ok_count += 1;
                        self.ch_tx_need_request
                            .check_capacity(0.2, "master_device | Response | ch_tx_buffer")
                            .send(())
                            .await
                            .map_err(|_| Error::TokioSyncMpscSend)?;
                    }
                    ResponseResult::Ok => device_state.response_ok_count += 1,
                    ResponseResult::Error(_) => device_state.response_err_count += 1,
                },
                Err(e) => {
                    warn!("Error in fn_response_to_buffer: {:?}", e);
                }
            };
        }

        Ok(())
    }
}
