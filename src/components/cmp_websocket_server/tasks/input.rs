use tokio::sync::{broadcast, mpsc};

use crate::{
    components::cmp_websocket_server::ServerToClientCache,
    components_config::websocket_server::{FnInput, WebsocketMessage},
    message::{Message, MsgDataBound},
};

pub struct Input<TMsg, TServerToClient>
where
    TMsg: MsgDataBound,
    TServerToClient: WebsocketMessage,
{
    pub input: mpsc::Receiver<Message<TMsg>>,
    pub output: broadcast::Sender<TServerToClient>,
    pub fn_input: FnInput<TMsg, TServerToClient>,
    pub cache: ServerToClientCache<TServerToClient>,
}

impl<TMsg, TServerToClient> Input<TMsg, TServerToClient>
where
    TMsg: MsgDataBound,
    TServerToClient: WebsocketMessage,
{
    pub async fn spawn(mut self) -> super::Result<()> {
        while let Some(msg) = self.input.recv().await {
            let s_to_c = (self.fn_input)(&msg);
            let Some(s_to_c) = s_to_c else { continue };

            // Сохраняем в кеше
            let key: &'static str = s_to_c.clone().into();
            {
                let mut cache = self.cache.lock().await;
                cache.insert(key.to_string(), s_to_c.clone());
            }

            // Отправляем подключенным клиентам
            self.output
                .send(s_to_c)
                .map_err(|_| super::Error::TokioSyncMpsc)?;
        }

        Err(super::Error::TaskEndInput)
    }
}
