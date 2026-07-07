use tokio::sync::mpsc::{self, error::TrySendError};
use tracing::warn;

use crate::{
    components_config::mqtt_client::{ConfigPublish, MqttMsgGen, MqttMsgSend},
    executor::{MsgBusInput, MsgBusLinker},
    message::MsgDataBound,
};

pub struct Input<TMsg, TError>
where
    TMsg: MsgDataBound,
{
    pub input: MsgBusInput<TMsg>,
    pub output: mpsc::Sender<MqttMsgSend>,
    pub config_publish: ConfigPublish<TMsg>,
    pub mqtt_msg_gen: MqttMsgGen,
    pub error_fn_publish: fn(String, anyhow::Error) -> TError,
    pub error_task_end: fn() -> TError,
    pub error_tokio_mpsc_send: fn() -> TError,
}

impl<TMsg, TError> Input<TMsg, TError>
where
    TMsg: MsgDataBound,
{
    pub async fn spawn(mut self) -> Result<(), TError> {
        let fn_publish = match self.config_publish {
            ConfigPublish::NoPublish => return Ok(()),
            ConfigPublish::Publish { fn_publish, .. } => fn_publish,
        };

        let mut channel_full = false;

        while let Ok(msg) = self.input.recv().await {
            let Some(msg) = msg.get_custom_data() else {
                continue;
            };

            let mqtt_msg = fn_publish(&msg, &self.mqtt_msg_gen)
                .map_err(|e| (self.error_fn_publish)(format!("{:?}", msg), e))?;

            let Some(mqtt_msg) = mqtt_msg else { continue };

            let res = self.output.try_send(mqtt_msg);
            if let Err(err) = res {
                match err {
                    TrySendError::Full(_) => {
                        if !channel_full {
                            channel_full = true;
                            warn!("MQTT client too slow or disconnected");
                            continue;
                        }
                    }
                    TrySendError::Closed(_) => {
                        break;
                    }
                }
            } else {
                channel_full = false;
            }
        }

        Err((self.error_task_end)())
    }
}
