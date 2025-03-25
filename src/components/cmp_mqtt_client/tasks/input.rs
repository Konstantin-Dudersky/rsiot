use rumqttc::AsyncClient;

use crate::{
    components::shared_mqtt_client::{create_payload_for_message, create_topic_for_message},
    components_config::mqtt_client::ConfigFnInput,
    executor::CmpInOut,
    message::MsgDataBound,
};

use super::shared::publish_on_broker;

pub struct Input<TMsg>
where
    TMsg: MsgDataBound,
{
    pub in_out: CmpInOut<TMsg>,
    pub config_fn_input: ConfigFnInput<TMsg>,
    pub client: AsyncClient,
}

impl<TMsg> Input<TMsg>
where
    TMsg: MsgDataBound,
{
    pub async fn spawn(mut self) -> super::Result<()> {
        {
            while let Ok(msg) = self.in_out.recv_input().await {
                let topic = create_topic_for_message(&msg);

                let payload = create_payload_for_message(&msg, self.config_fn_input);
                let Some(payload) = payload else { continue };

                publish_on_broker(topic, payload, &self.client).await?;
            }
            Ok(())
        }
    }
}
