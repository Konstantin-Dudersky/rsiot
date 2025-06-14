use rsiot::{
    components::cmp_mqtt_client::*, executor::Component, message::Message,
    serde_utils::SerdeAlgKind,
};

use crate::message::Custom;

pub fn cmp() -> Component<Config<Custom>, Custom> {
    let config = Config {
        client_id: "subscribe".into(),
        host: "localhost".into(),
        port: 1883,
        serde_alg: SerdeAlgKind::Json,
        client_capacity: 100,
        publish: ConfigPublish::NoPublish,
        subscribe: ConfigSubscribe::Subscribe {
            token: "example/#".into(),
            fn_subscribe: |mqtt_msg, mqtt_msg_gen| {
                let msg = match mqtt_msg.topic.as_str() {
                    "example/counter" => {
                        let v: i32 = mqtt_msg_gen.de(&mqtt_msg.payload)?;
                        Message::new_custom(Custom::Counter(v))
                    }
                    _ => return Ok(None),
                };
                Ok(Some(vec![msg]))
            },
        },
    };

    Cmp::new(config)
}
