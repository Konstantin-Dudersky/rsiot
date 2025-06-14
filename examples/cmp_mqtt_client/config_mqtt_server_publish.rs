use rsiot::{components::cmp_mqtt_client::*, executor::Component, serde_utils::SerdeAlgKind};

use crate::message::Custom;

pub fn cmp() -> Component<Config<Custom>, Custom> {
    let config = Config {
        serde_alg: SerdeAlgKind::Json,
        client_id: "pubish".into(),
        host: "localhost".into(),
        port: 1883,
        client_capacity: 100,
        publish: ConfigPublish::Publish {
            fn_publish: |msg, mqtt_msg_gen| {
                let mqtt_msg = match msg {
                    Custom::Counter(v) => mqtt_msg_gen.ser("example/counter", true, v)?,
                };
                Ok(Some(mqtt_msg))
            },
        },
        subscribe: ConfigSubscribe::NoSubscribe,
    };

    Cmp::new(config)
}
