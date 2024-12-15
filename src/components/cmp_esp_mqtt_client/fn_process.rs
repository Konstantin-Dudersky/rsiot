use std::time::Duration;

use esp_idf_svc::mqtt::client::{EspAsyncMqttClient, MqttClientConfiguration};
use tokio::{task::JoinSet, time::sleep};
use tracing::{info, warn};

use crate::{
    executor::CmpInOut,
    message::{system_messages, MsgData, MsgDataBound, ServiceBound},
};

use super::{tasks, Config};

pub async fn fn_process<TMsg, TService>(
    config: Config<TMsg>,
    mut in_out: CmpInOut<TMsg, TService>,
) -> super::Result<()>
where
    TMsg: MsgDataBound + 'static,
    TService: ServiceBound + 'static,
{
    // Необходимо подождать, пока поднимется Wi-Fi
    while let Ok(msg) = in_out.recv_input().await {
        match msg.data {
            MsgData::System(system_messages::System::EspWifiConnected) => break,
            _ => continue,
        }
    }
    main_loop(config, in_out).await?;
    Ok(())
}

async fn main_loop<TMsg, TService>(
    config: Config<TMsg>,
    in_out: CmpInOut<TMsg, TService>,
) -> super::Result<()>
where
    TMsg: MsgDataBound + 'static,
    TService: ServiceBound + 'static,
{
    loop {
        info!("Starting MQTT");

        let url = format!("mqtt://{}:{}", config.host, config.port);
        let conf = MqttClientConfiguration {
            client_id: Some(&config.client_id),
            keep_alive_interval: Some(Duration::from_secs(5)),
            ..Default::default()
        };

        let (client, connection) = EspAsyncMqttClient::new(&url, &conf).unwrap();
        info!("MQTT client created");

        let mut task_set: JoinSet<super::Result<()>> = JoinSet::new();

        // Получение сообщения от MQTT-брокера
        let task = tasks::Output {
            connection,
            config_fn_output: config.fn_output,
            in_out: in_out.clone(),
        };
        task_set.spawn_local(task.spawn());

        // Отправление сообщений из кеша на MQTT-брокер

        // Отправление входящих сообщений на MQTT-брокер
        let task = tasks::Input {
            in_out: in_out.clone(),
            config_fn_input: config.fn_input,
            client,
        };
        task_set.spawn_local(task.spawn());

        while let Some(res) = task_set.join_next().await {
            let res = res?;
            match res {
                Ok(_) => continue,
                Err(err) => {
                    let err = format!("cmp_esp_mqtt_client error: {}", err);
                    warn!("{err}");
                    break;
                }
            }
        }
        task_set.shutdown().await;

        sleep(Duration::from_secs(2)).await;
        // TODO - здесь все падает, ESP перезагружается.
        // Вроде все правильно, возможно ошибка в esp-idf-svc
    }
}
