use std::time::Duration;

use rumqttc::mqttbytes::QoS;
use rumqttc::{AsyncClient, MqttOptions};
use tokio::{task::JoinSet, time::sleep};
use tracing::{error, info};

use crate::message::ServiceBound;
use crate::{executor::CmpInOut, message::MsgDataBound};

use super::{tasks, Config};

pub async fn fn_process<TMsg, TService>(
    config: Config<TMsg>,
    in_out: CmpInOut<TMsg, TService>,
) -> super::Result<()>
where
    TMsg: MsgDataBound + 'static,
    TService: ServiceBound + 'static,
{
    loop {
        info!("Starting");
        let res = main(config.clone(), in_out.clone()).await;
        match res {
            Ok(_) => (),
            Err(err) => {
                error!("Error in cmp_mqtt_client: {}", err);
            }
        }
        info!("Restarting...");
        sleep(Duration::from_secs(2)).await;
    }
}

async fn main<TMsg, TService>(
    config: Config<TMsg>,
    in_out: CmpInOut<TMsg, TService>,
) -> super::Result<()>
where
    TMsg: MsgDataBound + 'static,
    TService: ServiceBound + 'static,
{
    let mut mqttoptions = MqttOptions::new(config.client_id, config.host, config.port);
    mqttoptions.set_keep_alive(Duration::from_secs(50000)); // TODO - прерывает обмен

    let (client, eventloop) = AsyncClient::new(mqttoptions, 10);
    client.subscribe("rsiot/#", QoS::ExactlyOnce).await?;

    let mut task_set: JoinSet<super::Result<()>> = JoinSet::new();

    // Отправление сообщений из кеша на MQTT-брокер
    let task = tasks::SendCache {
        in_out: in_out.clone(),
        config_fn_input: config.fn_input,
        client: client.clone(),
    };
    task_set.spawn(task.spawn());

    // Отправление входящих сообщений на MQTT-брокер
    let task = tasks::Input {
        in_out: in_out.clone(),
        config_fn_input: config.fn_input,
        client,
    };
    task_set.spawn(task.spawn());

    // Получение сообщения от MQTT-брокера
    let task = tasks::Output {
        in_out,
        config_fn_output: config.fn_output,
        eventloop,
    };
    task_set.spawn(task.spawn());

    while let Some(res) = task_set.join_next().await {
        res??
    }

    Ok(())
}
