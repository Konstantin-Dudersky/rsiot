use rumqttc::{AsyncClient, Event, EventLoop, Packet, Publish, QoS};
use tracing::{info, warn};

use crate::{
    components_config::mqtt_client::{ConfigSubscribe, FnSubscribe, MqttMsg, MqttMsgGen},
    executor::CmpInOut,
    message::MsgDataBound,
};

pub struct Subscribe<TMsg>
where
    TMsg: MsgDataBound,
{
    pub msg_bus: CmpInOut<TMsg>,
    pub client: AsyncClient,
    pub eventloop: EventLoop,
    pub mqtt_msg_gen: MqttMsgGen,
    pub subscribe: ConfigSubscribe<TMsg>,
}

impl<TMsg> Subscribe<TMsg>
where
    TMsg: MsgDataBound,
{
    pub async fn spawn(mut self) -> super::Result<()> {
        let mut trace = TraceMsg::new();

        loop {
            let notification = self.eventloop.poll().await;

            let notification = match notification {
                Ok(v) => {
                    trace.ok();
                    v
                }
                Err(e) => {
                    trace.error(e.to_string());
                    continue;
                }
            };

            match notification {
                Event::Incoming(msg) => match msg {
                    Packet::ConnAck(_) => {
                        if let ConfigSubscribe::Subscribe { token, .. } = &self.subscribe {
                            handle_event_connack(token, &self.client).await?;
                        }
                    }
                    Packet::Publish(msg) => {
                        if let ConfigSubscribe::Subscribe { fn_subscribe, .. } = self.subscribe {
                            handle_event_publish(
                                msg,
                                fn_subscribe,
                                &self.mqtt_msg_gen,
                                &self.msg_bus,
                            )
                            .await?
                        }
                    }
                    _ => (),
                },
                Event::Outgoing(_) => (),
            };
        }
    }
}

async fn handle_event_connack(topic: &str, client: &AsyncClient) -> super::Result<()> {
    client.subscribe(topic, QoS::ExactlyOnce).await?;
    Ok(())
}

/// Обработка события публикации сообщений в брокере другим клиентом
async fn handle_event_publish<TMsg>(
    msg: Publish,
    fn_subscribe: FnSubscribe<TMsg>,
    mqtt_msg_gen: &MqttMsgGen,
    msg_bus: &CmpInOut<TMsg>,
) -> super::Result<()>
where
    TMsg: MsgDataBound,
{
    let mqtt_msg = MqttMsg {
        topic: msg.topic,
        retain: msg.retain,
        payload: msg.payload.to_vec(),
    };
    let msgs = (fn_subscribe)(&mqtt_msg, mqtt_msg_gen);

    // Ошибка выполнения fn_output
    let msgs = match msgs {
        Ok(msg) => msg,
        Err(err) => {
            warn!("FnOutput: {err}");
            return Ok(());
        }
    };

    // Фильтруем сообщения
    let Some(msgs) = msgs else { return Ok(()) };

    // Отправляем исходящее сообщение
    for msg in msgs {
        msg_bus
            .send_output(msg)
            .await
            .map_err(super::Error::CmpOutput)?;
    }

    Ok(())
}

struct TraceMsg {
    state: u8,
}
impl TraceMsg {
    fn new() -> Self {
        Self { state: 0 }
    }
    fn error(&mut self, error: String) {
        if self.state != 1 {
            self.state = 1;
            warn!("cmp_mqtt_client; task subscription; {}", error);
        }
    }
    fn ok(&mut self) {
        if self.state != 2 {
            self.state = 2;
            info!("cmp_mqtt_client; task subscription; connection ok",);
        }
    }
}
