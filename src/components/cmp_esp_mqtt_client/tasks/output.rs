use esp_idf_svc::mqtt::client::{EspAsyncMqttConnection, EventPayload};
use tracing::warn;

use crate::{
    components_config::mqtt_client::ConfigFnOutput,
    executor::CmpInOut,
    message::{MsgDataBound, ServiceBound},
};

pub struct Output<TMsg, TService>
where
    TMsg: MsgDataBound,
    TService: ServiceBound,
{
    pub connection: EspAsyncMqttConnection,
    pub config_fn_output: ConfigFnOutput<TMsg>,
    pub in_out: CmpInOut<TMsg, TService>,
}

impl<TMsg, TService> Output<TMsg, TService>
where
    TMsg: MsgDataBound,
    TService: ServiceBound,
{
    pub async fn spawn(mut self) -> super::Result<()> {
        while let Ok(event) = self.connection.next().await {
            let data = match event.payload() {
                EventPayload::BeforeConnect => continue,
                EventPayload::Connected(_) => continue,
                EventPayload::Disconnected => {
                    return Err(super::Error::BrokerDisconnected);
                }
                EventPayload::Subscribed(_) => continue,
                EventPayload::Unsubscribed(_) => continue,
                EventPayload::Published(_) => continue,
                EventPayload::Received {
                    id: _,
                    topic: _,
                    data,
                    details: _,
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
