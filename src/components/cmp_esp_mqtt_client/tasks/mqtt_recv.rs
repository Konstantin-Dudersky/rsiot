use esp_idf_svc::{
    mqtt::client::{EspAsyncMqttConnection, EventPayload},
    sys::EspError,
};
use tokio::sync::mpsc;
use tracing::{info, trace, warn};

use crate::components_config::mqtt_client::MqttMsgRecv;

use super::Error;

pub struct MqttRecv {
    pub output: mpsc::Sender<MqttMsgRecv>,
    pub connection: EspAsyncMqttConnection,
}

const TEXT: &str = "\nMQTT Event";

impl MqttRecv {
    pub async fn spawn(mut self) -> super::Result<()> {
        while let Ok(event) = self.connection.next().await {
            let msg_int = process_mqtt_event(event.payload());

            if let Some(msg_int) = msg_int {
                self.output
                    .send(msg_int.clone())
                    .await
                    .map_err(|_| Error::TokioSyncMpscSend)?;
            }
        }
        Err(Error::TaskEndMqttRecv)
    }
}

fn process_mqtt_event(event_payload: EventPayload<EspError>) -> Option<MqttMsgRecv> {
    match event_payload {
        EventPayload::BeforeConnect => {
            info!("{TEXT}: BeforeConnect");
            None
        }

        EventPayload::Connected(flag) => {
            info!("{TEXT}: Connected: {flag}");
            Some(MqttMsgRecv::Connected)
        }

        EventPayload::Disconnected => {
            warn!("{TEXT}: Disconnected");
            Some(MqttMsgRecv::Disconnected)
        }

        EventPayload::Subscribed(id) => {
            info!("{TEXT}: Subscribed: id={id}");
            None
        }

        EventPayload::Unsubscribed(id) => {
            info!("{TEXT}: Unsubscribed: id={id}");
            None
        }

        EventPayload::Published(_) => None,

        EventPayload::Received {
            id: _,
            topic,
            data,
            details: _,
        } => {
            trace!("{TEXT}: Received: topic={topic:?}, data={data:?}");
            let topic = topic.unwrap_or("").to_string();
            Some(MqttMsgRecv::Received {
                topic,
                data: data.to_vec(),
            })
        }

        EventPayload::Deleted(_) => None,

        EventPayload::Error(err) => {
            warn!("{TEXT}: Error: {err}");
            None
        }
    }
}
