use std::time::Duration;

use esp_idf_svc::mqtt::client::{EspAsyncMqttClient, MqttClientConfiguration};
use tokio::task::JoinSet;
use tracing::info;

use crate::{
    components::shared_tasks::cmp_mqtt_genral::MqttGeneralTasks,
    components_config::mqtt_client::MqttMsgGen,
    executor::{MsgBusLinker, join_set_spawn},
    message::MsgDataBound,
    serde_utils::SerdeAlg,
};

use super::{Config, Error, tasks};

pub async fn fn_process<TMsg>(
    config: Config<TMsg>,
    msg_bus: MsgBusLinker<TMsg>,
) -> super::Result<()>
where
    TMsg: MsgDataBound + 'static,
{
    info!("Starting cmp_esp_mqtt_client");

    let buffer_size = msg_bus.max_capacity();

    let url = format!("mqtt://{}:{}", config.host, config.port);
    let conf = MqttClientConfiguration {
        client_id: Some(&config.client_id),
        keep_alive_interval: Some(Duration::from_secs(5)),
        ..Default::default()
    };

    let (client, connection) =
        EspAsyncMqttClient::new(&url, &conf).map_err(Error::CreateEspAsyncMqttClient)?;
    info!("MQTT client created");

    let mqtt_msg_gen = MqttMsgGen {
        serde_alg: SerdeAlg::new(config.serde_alg),
    };

    let mut task_set: JoinSet<super::Result<()>> = JoinSet::new();

    let (ch_rx_send, ch_tx_recv) = MqttGeneralTasks {
        msg_bus,
        buffer_size,
        task_set: &mut task_set,
        publish: config.publish,
        subscribe: config.subscribe,
        mqtt_msg_gen,
        error_fn_publish: Error::FnPublish,
        error_fn_subscribe: Error::FnSubscribe,
        error_task_end_input: || Error::TaskEndInput,
        error_task_end_output: || Error::TaskEndOutput,
        error_tokio_mpsc_send: || Error::TokioSyncMpscSend,
    }
    .spawn();

    // Получение сообщения от MQTT-брокера
    let task = tasks::MqttRecv {
        connection,
        output: ch_tx_recv,
    };
    join_set_spawn(
        &mut task_set,
        "cmp_esp_mqtt_client | mqtt_recv",
        task.spawn(),
    );

    // Отправление входящих сообщений на MQTT-брокер
    let task = tasks::MqttSend {
        input: ch_rx_send,
        client,
    };
    join_set_spawn(
        &mut task_set,
        "cmp_esp_mqtt_client | mqtt_send",
        task.spawn(),
    );

    while let Some(res) = task_set.join_next().await {
        res??
    }

    Err(Error::TaskEndMain)
}
