use rumqttc::{AsyncClient, QoS};
use tokio::sync::mpsc;

use crate::components_config::mqtt_client::MqttMsgSend;

use super::Error;

pub struct MqttSend {
    pub input: mpsc::Receiver<MqttMsgSend>,
    pub client: AsyncClient,
}

impl MqttSend {
    pub async fn spawn(mut self) -> super::Result<()> {
        while let Some(msg_int) = self.input.recv().await {
            match msg_int {
                MqttMsgSend::Publish {
                    topic,
                    retain,
                    payload,
                } => {
                    self.client
                        .publish(topic, QoS::ExactlyOnce, retain, payload)
                        .await?;
                }
                MqttMsgSend::Subscribe { topic } => {
                    self.client.subscribe(topic, QoS::ExactlyOnce).await?;
                }
            }
        }

        Err(Error::TaskEndMqttSend)
        // {
        //     loop {
        //         let msg = self.msg_bus.recv_input().await;

        //         let msg = match msg {
        //             Ok(v) => v,
        //             Err(e) => {
        //                 error!("{:?}", e);
        //                 continue;
        //             }
        //         };

        //         let Some(msg) = msg.get_custom_data() else {
        //             continue;
        //         };

        //         let mqtt_msg = (self.fn_publish)(&msg, &self.mqtt_msg_gen);
        //         let mqtt_msg = match mqtt_msg {
        //             Ok(v) => v,
        //             Err(e) => {
        //                 warn!("Error in MQTT fn_publish: {:?}", e);
        //                 continue;
        //             }
        //         };
        //         let Some(mqtt_msg) = mqtt_msg else { continue };

        //         trace!("Publish: {:?}", mqtt_msg);

        //         self.client
        //             .publish(mqtt_msg.topic, QoS::ExactlyOnce, true, mqtt_msg.payload)
        //             .await?;
        //     }
        // }
    }
}
