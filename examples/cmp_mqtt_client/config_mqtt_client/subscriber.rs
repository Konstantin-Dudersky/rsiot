#[cfg(feature = "cmp_esp")]
use rsiot::components::cmp_esp_mqtt_client::*;
#[cfg(feature = "cmp_mqtt_client")]
use rsiot::components::cmp_mqtt_client::*;
use rsiot::{executor::Component, serde_utils::SerdeAlgKind};

use crate::message::*;

pub fn cmp() -> Component<Config<Msg>, Msg> {
    let config = Config {
        serde_alg: SerdeAlgKind::Cbor,
        client_id: "subscriber".into(),
        // host: "192.168.101.12".into(),
        host: "localhost".into(),
        port: 1883,
        client_capacity: 100,
        publish: ConfigPublish::NoPublish,
        subscribe: ConfigSubscribe::Subscribe {
            topic: "example/counter".into(),
            fn_subscribe: |_topic, data, mqtt_msg_gen| {
                let data = mqtt_msg_gen.de(data)?;
                let msg = Msg::Subscribe(data);
                Ok(Some(vec![msg]))
            },
        },
    };

    Cmp::new(config)
}
