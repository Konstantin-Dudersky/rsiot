use std::time::Duration;

use esp_idf_svc::mqtt::client::{
    EspAsyncMqttClient, EspAsyncMqttConnection, EventPayload, MqttClientConfiguration, QoS,
};
use tokio::{task::JoinSet, time::sleep};
use tracing::{info, warn};

use crate::{executor::CmpInOut, message::MsgDataBound};

use super::{config::ConfigFnInput, Config};

pub async fn fn_process<TMsg>(config: Config<TMsg>, in_out: CmpInOut<TMsg>) -> super::Result<()>
where
    TMsg: MsgDataBound + 'static,
{
    // Необходимо подождать, пока поднимется Wi-Fi
    sleep(Duration::from_secs(2)).await;
    info!("Starting MQTT");

    let url = format!("mqtt://{}:{}", config.host, config.port);
    info!("Url: {url}");
    let conf = MqttClientConfiguration {
        client_id: Some(&config.client_id),
        keep_alive_interval: Some(Duration::from_secs(5)),
        ..Default::default()
    };

    let (client, connection) = EspAsyncMqttClient::new(&url, &conf).unwrap();
    info!("MQTT client created");

    let mut task_set: JoinSet<super::Result<()>> = JoinSet::new();

    task_set.spawn_local(input(in_out.clone(), config.fn_input, client));
    task_set.spawn_local(output(connection));

    while let Some(res) = task_set.join_next().await {
        res??
    }
    Ok(())
}

async fn input<TMsg>(
    mut in_out: CmpInOut<TMsg>,
    config_fn_input: ConfigFnInput<TMsg>,
    mut client: EspAsyncMqttClient,
) -> super::Result<()>
where
    TMsg: MsgDataBound,
{
    client.subscribe("rsiot/#", QoS::ExactlyOnce).await.unwrap();
    info!("MQTT client subscribed to topic");
    while let Ok(msg) = in_out.recv_input().await {
        let topic = msg.key.replace('-', "/").to_lowercase();
        let topic = format!("rsiot/{topic}");

        let payload = config_fn_input(msg);

        // Ошибка выполнения fn_input
        let payload = match payload {
            Ok(payload) => payload,
            Err(err) => {
                warn!("FnInput: {err}");
                continue;
            }
        };

        // Фильтруем сообщения
        let Some(payload) = payload else { continue };

        client
            .publish(&topic, QoS::ExactlyOnce, true, &payload)
            .await
            .unwrap();
    }
    Ok(())
}

async fn output(mut connection: EspAsyncMqttConnection) -> super::Result<()> {
    loop {
        let event = connection.next().await.unwrap();
        match event.payload() {
            EventPayload::BeforeConnect => continue,
            EventPayload::Connected(_) => continue,
            EventPayload::Disconnected => continue,
            EventPayload::Subscribed(_) => continue,
            EventPayload::Unsubscribed(_) => continue,
            EventPayload::Published(_) => continue,
            EventPayload::Received {
                id: _,
                topic: _,
                data: _,
                details: _,
            } => continue,
            EventPayload::Deleted(_) => continue,
            EventPayload::Error(err) => {
                warn!("{err}");
                continue;
            }
        }
    }
}
