use rumqttc::AsyncClient;

use crate::{
    components::shared_mqtt_client::{create_payload_for_message, create_topic_for_message},
    components_config::mqtt_client::ConfigFnInput,
    executor::CmpInOut,
    message::MsgDataBound,
};

use super::shared::publish_on_broker;

pub struct SendCache<TMsg>
where
    TMsg: MsgDataBound,
{
    pub in_out: CmpInOut<TMsg>,
    pub config_fn_input: ConfigFnInput<TMsg>,
    pub client: AsyncClient,
}

impl<TMsg> SendCache<TMsg>
where
    TMsg: MsgDataBound,
{
    pub async fn spawn(self) -> super::Result<()> {
        let cache = self.in_out.cache.read().await;

        for (_, msg) in cache.iter() {
            let topic = create_topic_for_message(msg);

            let payload = create_payload_for_message(msg, self.config_fn_input);
            let payload = match payload {
                Some(payload) => payload,
                None => continue,
            };

            publish_on_broker(topic, payload, &self.client).await?;
        }

        Ok(())
    }
}
