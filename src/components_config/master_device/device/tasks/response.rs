use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

use tokio::sync::mpsc;
use tracing::{trace, warn};

use crate::{
    executor::CheckCapacity,
    message::{Message, MsgDataBound},
};

use super::{Buffer, Error, FieldbusDiagMsg, RequestResponseBound, ResponseResult};

pub struct Response<TMsg, TResponse, TBuffer> {
    pub device_id: String,
    pub buffer: Buffer<TBuffer>,
    pub init_completed: Arc<AtomicBool>,
    pub ch_rx_fieldbus_to_device: mpsc::Receiver<TResponse>,
    pub ch_tx_output_to_filter: mpsc::Sender<Message<TMsg>>,
    pub ch_tx_need_request: mpsc::Sender<()>,
    pub ch_tx_device_to_diag: mpsc::Sender<FieldbusDiagMsg>,
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

            let request_duration = response.request_duration();

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

            let diag_msg = match req_res {
                Ok(req_res) => match req_res {
                    ResponseResult::OkInitCompleted => {
                        self.init_completed.store(true, Ordering::Relaxed);
                        FieldbusDiagMsg::DeviceInitCompleted {
                            id: self.device_id.clone(),
                            duration: request_duration,
                        }
                    }
                    ResponseResult::OkNeedRequest => {
                        self.ch_tx_need_request
                            .check_capacity(0.2, "master_device | Response | ch_tx_buffer")
                            .send(())
                            .await
                            .map_err(|_| Error::TokioSyncMpscSend)?;
                        FieldbusDiagMsg::DeviceRequestOk {
                            id: self.device_id.clone(),
                            duration: request_duration,
                        }
                    }
                    ResponseResult::Ok => FieldbusDiagMsg::DeviceRequestOk {
                        id: self.device_id.clone(),
                        duration: request_duration,
                    },
                    ResponseResult::Error(err) => FieldbusDiagMsg::DeviceRequestErr {
                        id: self.device_id.clone(),
                        duration: request_duration,
                        error: err,
                    },
                },
                Err(err) => {
                    warn!("Error in fn_response_to_buffer: {:?}", err);

                    FieldbusDiagMsg::DeviceRequestErr {
                        id: self.device_id.clone(),
                        duration: request_duration,
                        error: err.to_string(),
                    }
                }
            };

            self.ch_tx_device_to_diag
                .check_capacity(0.2, "master_device | Response | ch_tx_device_to_diag")
                .send(diag_msg)
                .await
                .map_err(|_| Error::TokioSyncMpscSend)?;
        }

        Ok(())
    }
}
