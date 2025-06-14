use rumqttc::{AsyncClient, QoS};
use tracing::{error, trace, warn};

use crate::{
    components_config::mqtt_client::{FnPublish, MqttMsgGen},
    executor::CmpInOut,
    message::MsgDataBound,
};

pub struct Publish<TMsg>
where
    TMsg: MsgDataBound,
{
    pub msg_bus: CmpInOut<TMsg>,
    pub fn_publish: FnPublish<TMsg>,
    pub mqtt_msg_gen: MqttMsgGen,
    pub client: AsyncClient,
}

impl<TMsg> Publish<TMsg>
where
    TMsg: MsgDataBound,
{
    pub async fn spawn(mut self) -> super::Result<()> {
        {
            loop {
                let msg = self.msg_bus.recv_input().await;

                let msg = match msg {
                    Ok(v) => v,
                    Err(e) => {
                        error!("{:?}", e);
                        continue;
                    }
                };

                let Some(msg) = msg.get_custom_data() else {
                    continue;
                };

                let mqtt_msg = (self.fn_publish)(&msg, &self.mqtt_msg_gen);
                let mqtt_msg = match mqtt_msg {
                    Ok(v) => v,
                    Err(e) => {
                        warn!("Error in MQTT fn_publish: {:?}", e);
                        continue;
                    }
                };
                let Some(mqtt_msg) = mqtt_msg else { continue };

                trace!("Publish: {:?}", mqtt_msg);

                self.client
                    .publish(mqtt_msg.topic, QoS::ExactlyOnce, true, mqtt_msg.payload)
                    .await?;
            }
        }
    }
}
