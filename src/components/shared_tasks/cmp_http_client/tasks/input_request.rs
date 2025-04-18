use std::time::Duration;

use tokio::sync::{broadcast, mpsc};

use crate::{
    components_config::http_client::{MsgRequest, RequestInput},
    message::{Message, MsgDataBound},
};

use super::{Error, Result};

pub struct InputRequest<TMsg> {
    pub input: mpsc::Receiver<Message<TMsg>>,
    pub output: mpsc::Sender<MsgRequest>,
    pub request_input_config: Vec<Box<dyn RequestInput<TMsg>>>,
}

impl<TMsg> InputRequest<TMsg>
where
    TMsg: MsgDataBound,
{
    pub async fn spawn(mut self) -> Result<()> {
        while let Some(msg) = self.input.recv().await {
            let Some(msg) = msg.get_custom_data() else {
                continue;
            };

            for ric in self.request_input_config.iter_mut() {
                let request = ric.create_request(&msg);
                let Some(request) = request else { continue };
                self.output
                    .send(request)
                    .await
                    .map_err(|_| Error::TokioSyncMpscSend)?;
            }
        }

        Err(Error::TaskInputRequest)
    }
}
