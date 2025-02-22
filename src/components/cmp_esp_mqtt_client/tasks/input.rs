use std::time::Duration;

use esp_idf_svc::mqtt::client::{EspAsyncMqttClient, QoS};
use tokio::time::sleep;
use tracing::{info, warn};

use crate::{
    components::shared_mqtt_client::{create_payload_for_message, create_topic_for_message},
    components_config::mqtt_client::ConfigFnInput,
    executor::CmpInOut,
    message::{MsgDataBound, ServiceBound},
};

pub struct Input<TMsg, TService>
where
    TMsg: MsgDataBound,
    TService: ServiceBound,
{
    pub in_out: CmpInOut<TMsg, TService>,
    pub config_fn_input: ConfigFnInput<TMsg>,
    pub client: EspAsyncMqttClient,
}

impl<TMsg, TService> Input<TMsg, TService>
where
    TMsg: MsgDataBound,
    TService: ServiceBound,
{
    pub async fn spawn(mut self) -> super::Result<()> {
        let topic = "rsiot/#";
        loop {
            info!("MQTT client: trying to subscribe to topic");
            let res = self.client.subscribe(topic, QoS::ExactlyOnce).await;
            match res {
                Ok(_) => break,
                Err(err) => {
                    warn!("{}", err);
                }
            }
            sleep(Duration::from_secs(5)).await;
        }
        info!("MQTT client subscribed to topic");

        while let Ok(msg) = self.in_out.recv_input().await {
            let topic = create_topic_for_message(&msg);

            let payload = create_payload_for_message(&msg, self.config_fn_input);
            let Some(payload) = payload else { continue };

            self.client
                .publish(&topic, QoS::ExactlyOnce, true, &payload)
                .await
                .unwrap();
        }
        Ok(())
    }
}
