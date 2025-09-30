use esp_idf_svc::mqtt::client::{EspAsyncMqttClient, QoS};
use tokio::sync::mpsc;
use tracing::warn;

use crate::components_config::mqtt_client::MqttMsgSend;

use super::Error;

pub struct MqttSend {
    pub input: mpsc::Receiver<MqttMsgSend>,
    pub client: EspAsyncMqttClient,
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
                        .publish(&topic, QoS::ExactlyOnce, retain, &payload)
                        .await
                        .map_err(|_| Error::TokioSyncMpscSend)?;
                }
                MqttMsgSend::Subscribe { topic } => {
                    let res = self.client.subscribe(&topic, QoS::ExactlyOnce).await;
                    match res {
                        Ok(_) => (),
                        Err(err) => {
                            warn!("Subscribe error: {}", err);
                        }
                    }
                }
            }
        }

        Err(Error::TaskEndMqttSend)
    }
}
