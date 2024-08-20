use std::time::Duration;

use rumqttc::mqttbytes::QoS;
use rumqttc::{AsyncClient, Event, EventLoop, MqttOptions, Packet};
use tokio::{task::JoinSet, time::sleep};
use tracing::{error, info, warn};

use crate::{executor::CmpInOut, message::MsgDataBound};

use super::{
    config::{ConfigFnInput, ConfigFnOutput},
    Config,
};

pub async fn fn_process<TMsg>(config: Config<TMsg>, in_out: CmpInOut<TMsg>) -> super::Result<()>
where
    TMsg: MsgDataBound + 'static,
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

async fn main<TMsg>(config: Config<TMsg>, in_out: CmpInOut<TMsg>) -> super::Result<()>
where
    TMsg: MsgDataBound + 'static,
{
    let mut mqttoptions = MqttOptions::new(config.client_id, config.host, config.port);
    mqttoptions.set_keep_alive(Duration::from_secs(50000)); // TODO - прерывает обмен

    let (client, eventloop) = AsyncClient::new(mqttoptions, 10);
    client.subscribe("rsiot/#", QoS::ExactlyOnce).await?;

    let mut task_set: JoinSet<super::Result<()>> = JoinSet::new();

    task_set.spawn(input(in_out.clone(), config.fn_input, client));
    task_set.spawn(output(in_out, config.fn_output, eventloop));

    while let Some(res) = task_set.join_next().await {
        res??
    }

    Ok(())
}

/// Обработка входящих сообщений и публикация в брокере
async fn input<TMsg>(
    mut in_out: CmpInOut<TMsg>,
    config_fn_input: ConfigFnInput<TMsg>,
    client: AsyncClient,
) -> super::Result<()>
where
    TMsg: MsgDataBound,
{
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
            .publish(topic, QoS::ExactlyOnce, false, payload)
            .await?;
        // match res {
        //     Ok(_) => (),
        //     Err(err) => warn!("Error publishing to MQTT: {}", err),
        // }
    }
    Ok(())
}

/// Получение данных по подписке от брокера и преобразование в исходящие сообщения
async fn output<TMsg>(
    in_out: CmpInOut<TMsg>,
    config_fn_output: ConfigFnOutput<TMsg>,
    mut eventloop: EventLoop,
) -> super::Result<()>
where
    TMsg: MsgDataBound,
{
    while let Ok(notification) = eventloop.poll().await {
        let msg = match notification {
            Event::Incoming(msg) => match msg {
                Packet::Connect(_) => continue,
                Packet::ConnAck(_) => continue,
                Packet::Publish(msg) => msg,
                Packet::PubAck(_) => continue,
                Packet::PubRec(_) => continue,
                Packet::PubRel(_) => continue,
                Packet::PubComp(_) => continue,
                Packet::Subscribe(_) => continue,
                Packet::SubAck(_) => continue,
                Packet::Unsubscribe(_) => continue,
                Packet::UnsubAck(_) => continue,
                Packet::PingReq => continue,
                Packet::PingResp => continue,
                Packet::Disconnect => continue,
            },
            Event::Outgoing(_) => continue,
        };
        let payload = msg.payload.to_vec();

        let msg = config_fn_output(&payload);

        // Ошибка выполнения fn_output
        let msg = match msg {
            Ok(msg) => msg,
            Err(err) => {
                warn!("FnOutput: {err}");
                continue;
            }
        };

        // Фильтруем сообщения
        let Some(msg) = msg else { continue };

        // Отправляем исходящее сообщение
        in_out
            .send_output(msg)
            .await
            .map_err(super::Error::CmpOutput)?;
    }
    Ok(())
}
