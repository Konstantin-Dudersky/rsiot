use std::time::Duration;

use rumqttc::{AsyncClient, MqttOptions};
use tokio::task::JoinSet;
use tracing::info;

use crate::{
    components::shared_tasks::cmp_mqtt_genral::MqttGeneralTasks,
    executor::{CmpInOut, join_set_spawn},
    message::MsgDataBound,
    serde_utils::SerdeAlg,
};

use super::{Config, Error, config::MqttMsgGen, tasks};

pub async fn fn_process<TMsg>(config: Config<TMsg>, msg_bus: CmpInOut<TMsg>) -> super::Result<()>
where
    TMsg: MsgDataBound + 'static,
{
    info!("Starting");

    let buffer_size = msg_bus.max_capacity();

    let mut mqttoptions = MqttOptions::new(&config.client_id, &config.host, config.port);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (client, eventloop) = AsyncClient::new(mqttoptions, config.client_capacity);

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

    // Отправление входящих сообщений на MQTT-брокер
    let task = tasks::MqttSend {
        input: ch_rx_send,
        client: client.clone(),
    };
    join_set_spawn(&mut task_set, "cmp_mqtt_client | mqtt_send", task.spawn());

    // Получение сообщения от MQTT-брокера
    let task = tasks::MqttRecv {
        output: ch_tx_recv,
        eventloop,
    };
    join_set_spawn(&mut task_set, "cmp_mqtt_client", task.spawn());

    while let Some(res) = task_set.join_next().await {
        res??
    }

    Err(Error::TaskEndMain)
}
