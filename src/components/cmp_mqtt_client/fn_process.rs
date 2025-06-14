use std::time::Duration;

use rumqttc::{AsyncClient, MqttOptions};
use tokio::task::JoinSet;
use tracing::info;

use crate::components_config::mqtt_client::ConfigPublish;
use crate::executor::join_set_spawn;
use crate::serde_utils::SerdeAlg;
use crate::{executor::CmpInOut, message::MsgDataBound};

use super::config::MqttMsgGen;
use super::{tasks, Config};

pub async fn fn_process<TMsg>(config: Config<TMsg>, msg_bus: CmpInOut<TMsg>) -> super::Result<()>
where
    TMsg: MsgDataBound + 'static,
{
    loop {
        info!("Starting");

        let mut mqttoptions = MqttOptions::new(&config.client_id, &config.host, config.port);
        mqttoptions.set_keep_alive(Duration::from_secs(5));

        let (client, eventloop) = AsyncClient::new(mqttoptions, config.client_capacity);

        let mqtt_msg_gen = MqttMsgGen {
            serde_alg: SerdeAlg::new(config.serde_alg),
        };

        let mut task_set: JoinSet<super::Result<()>> = JoinSet::new();

        // Отправление входящих сообщений на MQTT-брокер
        if let ConfigPublish::Publish { fn_publish } = config.publish {
            let task = tasks::Publish {
                msg_bus: msg_bus.clone(),
                fn_publish,
                mqtt_msg_gen: mqtt_msg_gen.clone(),
                client: client.clone(),
            };
            join_set_spawn(&mut task_set, task.spawn());
        }

        // Получение сообщения от MQTT-брокера
        let task = tasks::Subscribe {
            msg_bus: msg_bus.clone(),
            eventloop,
            client,
            mqtt_msg_gen: mqtt_msg_gen.clone(),
            subscribe: config.subscribe.clone(),
        };
        join_set_spawn(&mut task_set, task.spawn());

        while let Some(res) = task_set.join_next().await {
            res??
        }
    }
}
