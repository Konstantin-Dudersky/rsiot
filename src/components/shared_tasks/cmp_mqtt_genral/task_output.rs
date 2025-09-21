use tokio::sync::mpsc;

use crate::{
    components_config::mqtt_client::{ConfigSubscribe, MqttMsgGen, MqttMsgRecv, MqttMsgSend},
    executor::CmpInOut,
    message::{Message, MsgDataBound},
};

pub struct Output<TMsg, TError>
where
    TMsg: MsgDataBound,
{
    pub input: mpsc::Receiver<MqttMsgRecv>,
    pub output_send: mpsc::Sender<MqttMsgSend>,
    pub output_msg_bus: CmpInOut<TMsg>,
    pub config_subscribe: ConfigSubscribe<TMsg>,
    pub mqtt_msg_gen: MqttMsgGen,
    pub error_fn_subscribe: fn(anyhow::Error) -> TError,
    pub error_tokio_mpsc_send: fn() -> TError,
    pub error_task_end: fn() -> TError,
}

impl<TMsg, TError> Output<TMsg, TError>
where
    TMsg: MsgDataBound,
{
    pub async fn spawn(mut self) -> Result<(), TError> {
        while let Some(msg_int_output) = self.input.recv().await {
            match msg_int_output {
                MqttMsgRecv::Connected => {
                    let ConfigSubscribe::Subscribe {
                        topic,
                        fn_subscribe: _,
                    } = self.config_subscribe.clone()
                    else {
                        continue;
                    };
                    let msg_int_input = MqttMsgSend::Subscribe { topic };
                    self.output_send
                        .send(msg_int_input)
                        .await
                        .map_err(|_| (self.error_tokio_mpsc_send)())?;
                }
                MqttMsgRecv::Disconnected => (),
                MqttMsgRecv::Received { topic, data } => {
                    let ConfigSubscribe::Subscribe {
                        topic: _,
                        fn_subscribe,
                    } = self.config_subscribe.clone()
                    else {
                        continue;
                    };

                    let msgs = fn_subscribe(&topic, &data, &self.mqtt_msg_gen)
                        .map_err(self.error_fn_subscribe)?;
                    let Some(msgs) = msgs else { continue };

                    for msg in msgs {
                        let msg = Message::new_custom(msg);
                        self.output_msg_bus
                            .send_output(msg)
                            .await
                            .map_err(|_| (self.error_tokio_mpsc_send)())?;
                    }
                }
            }
        }

        Err((self.error_task_end)())
    }
}
