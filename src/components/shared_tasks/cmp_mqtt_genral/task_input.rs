use tokio::sync::mpsc;

use crate::{
    components_config::mqtt_client::{ConfigPublish, MqttMsgGen, MqttMsgSend},
    executor::{CmpInOut, MsgBusInput},
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
    pub error_fn_publish: fn(anyhow::Error) -> TError,
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
            ConfigPublish::Publish { fn_publish } => fn_publish,
        };

        while let Ok(msg) = self.input.recv().await {
            let Some(msg) = msg.get_custom_data() else {
                continue;
            };

            let mqtt_msg = fn_publish(&msg, &self.mqtt_msg_gen).map_err(self.error_fn_publish)?;

            let Some(mqtt_msg) = mqtt_msg else { continue };

            self.output
                .send(mqtt_msg)
                .await
                .map_err(|_| (self.error_tokio_mpsc_send)())?;
        }

        Err((self.error_task_end)())
    }
}
