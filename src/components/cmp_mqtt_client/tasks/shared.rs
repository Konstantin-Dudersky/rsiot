use rumqttc::{AsyncClient, QoS};

pub async fn publish_on_broker(
    topic: String,
    payload: Vec<u8>,
    client: &AsyncClient,
) -> super::Result<()> {
    client
        .publish(topic, QoS::ExactlyOnce, true, payload)
        .await?;
    Ok(())
}
