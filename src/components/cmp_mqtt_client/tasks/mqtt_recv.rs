use rumqttc::{Event, EventLoop, Packet};
use tokio::sync::mpsc;
use tracing::{info, warn};

use crate::components_config::mqtt_client::MqttMsgRecv;

use super::Error;

pub struct MqttRecv {
    pub output: mpsc::Sender<MqttMsgRecv>,
    pub eventloop: EventLoop,
}

impl MqttRecv {
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

            let mqtt_msg_recv = match notification {
                Event::Incoming(packet) => process_mqtt_event(packet),
                Event::Outgoing(_) => continue,
            };

            if let Some(mqtt_msg_recv) = mqtt_msg_recv {
                self.output
                    .send(mqtt_msg_recv.clone())
                    .await
                    .map_err(|_| Error::TokioSyncMpscSend)?;
            }
        }
    }
}

fn process_mqtt_event(packet: Packet) -> Option<MqttMsgRecv> {
    match packet {
        Packet::Connect(_) => None,
        Packet::ConnAck(_) => Some(MqttMsgRecv::Connected),
        Packet::Publish(packet) => {
            let msg = MqttMsgRecv::Received {
                topic: packet.topic,
                data: packet.payload.to_vec(),
            };
            Some(msg)
        }
        Packet::PubAck(_) => None,
        Packet::PubRec(_) => None,
        Packet::PubRel(_) => None,
        Packet::PubComp(_) => None,
        Packet::Subscribe(_) => None,
        Packet::SubAck(_) => None,
        Packet::Unsubscribe(_) => None,
        Packet::UnsubAck(_) => None,
        Packet::PingReq => None,
        Packet::PingResp => None,
        Packet::Disconnect => None,
    }
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
