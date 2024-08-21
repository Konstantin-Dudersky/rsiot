use rumqttc::{Event, EventLoop, Packet};
use tracing::warn;

use crate::{
    components_config::mqtt_client::ConfigFnOutput, executor::CmpInOut, message::MsgDataBound,
};

pub struct Output<TMsg>
where
    TMsg: MsgDataBound,
{
    pub in_out: CmpInOut<TMsg>,
    pub config_fn_output: ConfigFnOutput<TMsg>,
    pub eventloop: EventLoop,
}

impl<TMsg> Output<TMsg>
where
    TMsg: MsgDataBound,
{
    pub async fn spawn(mut self) -> super::Result<()> {
        while let Ok(notification) = self.eventloop.poll().await {
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

            let msg = (self.config_fn_output)(&payload);

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
            self.in_out
                .send_output(msg)
                .await
                .map_err(super::Error::CmpOutput)?;
        }
        Ok(())
    }
}
