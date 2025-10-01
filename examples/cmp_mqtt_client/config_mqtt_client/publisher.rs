#[cfg(feature = "cmp_esp")]
use rsiot::components::cmp_esp_mqtt_client::*;
#[cfg(feature = "cmp_mqtt_client")]
use rsiot::components::cmp_mqtt_client::*;
use rsiot::{executor::Component, serde_utils::SerdeAlgKind};

use crate::message::*;

pub fn cmp() -> Component<Config<Msg>, Msg> {
    let config = Config {
        serde_alg: SerdeAlgKind::Cbor,
        client_id: "pubisher".into(),
        // host: "192.168.101.12".into(),
        host: "localhost".into(),
        port: 1883,
        client_capacity: 100,
        publish: ConfigPublish::Publish {
            base_topic: "example".into(),
            fn_publish: |msg, mqtt_msg_gen| {
                let mqtt_msg = match msg {
                    Msg::Counter(v) => mqtt_msg_gen.ser("counter", true, v)?,
                    _ => return Ok(None),
                };
                Ok(Some(mqtt_msg))
            },
        },
        subscribe: ConfigSubscribe::NoSubscribe,
    };

    Cmp::new(config)
}
