use esp_idf_svc::mqtt::client::{EspAsyncMqttConnection, EventPayload};
use tracing::{info, warn};

use crate::{
    components_config::mqtt_client::ConfigFnOutput, executor::CmpInOut, message::MsgDataBound,
};

pub struct Output<TMsg> {
    pub connection: EspAsyncMqttConnection,
    pub config_fn_output: ConfigFnOutput<TMsg>,
    pub in_out: CmpInOut<TMsg>,
}

impl<TMsg> Output<TMsg>
where
    TMsg: MsgDataBound,
{
    pub async fn spawn(mut self) -> super::Result<()> {
        while let Ok(event) = self.connection.next().await {
            let data = match event.payload() {
                EventPayload::BeforeConnect => continue,
                EventPayload::Connected(_) => continue,
                EventPayload::Disconnected => {
                    warn!("disconnected");
                    continue;
                }
                EventPayload::Subscribed(_) => continue,
                EventPayload::Unsubscribed(_) => continue,
                EventPayload::Published(_) => continue,
                EventPayload::Received {
                    id,
                    topic,
                    data,
                    details,
                } => data,
                EventPayload::Deleted(_) => continue,
                EventPayload::Error(err) => {
                    warn!("{err}");
                    continue;
                }
            };

            let payload = data.to_vec();

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
            self.in_out.send_output(msg).await.unwrap();
        }
        Ok(())
    }
}
