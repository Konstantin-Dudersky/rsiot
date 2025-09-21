use std::time::Duration;

use esp_idf_svc::mqtt::client::{EspAsyncMqttClient, MqttClientConfiguration};
use tokio::task::JoinSet;
use tracing::info;

use crate::{
    components::shared_tasks::cmp_mqtt_genral::MqttGeneralTasks,
    components_config::mqtt_client::MqttMsgGen,
    executor::{CmpInOut, join_set_spawn},
    message::MsgDataBound,
    serde_utils::SerdeAlg,
};

use super::{Config, Error, tasks};

pub async fn fn_process<TMsg>(config: Config<TMsg>, in_out: CmpInOut<TMsg>) -> super::Result<()>
where
    TMsg: MsgDataBound + 'static,
{
    info!("Starting cmp_esp_mqtt_client");
    main_loop(config, in_out).await?;
    Ok(())
}

async fn main_loop<TMsg>(config: Config<TMsg>, msg_bus: CmpInOut<TMsg>) -> super::Result<()>
where
    TMsg: MsgDataBound + 'static,
{
    info!("Starting MQTT");

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
        buffer_size: 100,
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

    // Преобразование входящих сообщений в данные для публикации MQTT
    // let task = tasks::Input {
    //     input: msg_bus.clone(),
    //     output: ch_tx_input.clone(),
    //     config_publish: config.publish,
    //     mqtt_msg_gen: mqtt_msg_gen.clone(),
    // };
    // join_set_spawn(&mut task_set, "cmp_esp_mqtt_client | input", task.spawn());

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

    // let task = tasks::Output {
    //     input: ch_rx_output,
    //     output_send: ch_tx_input,
    //     output_msg_bus: msg_bus,
    //     config_subscribe: config.subscribe,
    //     mqtt_msg_gen: mqtt_msg_gen.clone(),
    // };
    // join_set_spawn(&mut task_set, "cmp_esp_mqtt_client | output", task.spawn());

    while let Some(res) = task_set.join_next().await {
        res??
    }

    Err(Error::TaskEndMain)
}
