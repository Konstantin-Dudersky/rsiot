use rumqttc::{AsyncClient, QoS};
use tracing::warn;

use crate::{
    components_config::mqtt_client::ConfigFnInput,
    message::{Message, MsgDataBound},
};

pub fn create_topic_for_message<TMsg>(msg: &Message<TMsg>) -> String {
    let topic = msg.key.replace('-', "/").to_lowercase();
    let topic = format!("rsiot/{topic}");
    topic
}

pub fn create_payload_for_message<TMsg>(
    msg: &Message<TMsg>,
    config_fn_input: ConfigFnInput<TMsg>,
) -> Option<Vec<u8>>
where
    TMsg: MsgDataBound,
{
    let payload = (config_fn_input)(msg);

    // Ошибка выполнения fn_input
    let payload = match payload {
        Ok(payload) => payload,
        Err(err) => {
            warn!("FnInput: {err}");
            return None;
        }
    };

    // Фильтрация сообщений
    let payload = match payload {
        Some(payload) => payload,
        None => {
            return None;
        }
    };

    Some(payload)
}

pub async fn publish_on_broker(
    topic: String,
    payload: Vec<u8>,
    client: &AsyncClient,
) -> super::Result<()> {
    client
        .publish(topic, QoS::ExactlyOnce, false, payload)
        .await?;
    Ok(())
}
