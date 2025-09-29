use std::collections::HashMap;

use gloo::storage::{LocalStorage, SessionStorage, Storage};
use tracing::warn;

use crate::{
    executor::MsgBusOutput,
    message::{Message, MsgDataBound},
};

use super::{
    super::{ConfigStorageKind, config::FnOutput},
    TaskInput,
};

pub struct Output<TMsg>
where
    TMsg: MsgDataBound,
{
    pub input: TaskInput<TMsg>,
    pub output: MsgBusOutput<TMsg>,
    pub storage_kind: ConfigStorageKind,
    pub default_messages: Vec<Message<TMsg>>,
    pub fn_output: FnOutput<TMsg>,
}

impl<TMsg> Output<TMsg>
where
    TMsg: MsgDataBound,
{
    pub async fn spawn(mut self) -> super::Result<()> {
        // Загружаем из хранилища все значения
        let msgs: Result<HashMap<String, Message<TMsg>>, _> = match self.storage_kind {
            ConfigStorageKind::LocalStorage => LocalStorage::get_all(),
            ConfigStorageKind::SessionStorage => SessionStorage::get_all(),
        };

        let mut msgs = match msgs {
            Ok(val) => val,
            Err(err) => {
                warn!("Error loading messages: {}", err);
                match self.storage_kind {
                    ConfigStorageKind::LocalStorage => LocalStorage::clear(),
                    ConfigStorageKind::SessionStorage => SessionStorage::clear(),
                }
                warn!("Storage {:?} cleared", self.storage_kind);
                HashMap::new()
            }
        };

        // Добавляем значения по-умолчанию
        for msg in self.default_messages {
            if !msgs.contains_key(&msg.key) {
                msgs.insert(msg.key.clone(), msg);
            }
        }

        // Фильтруем сообщения на основе fn_output и отправляем исходящие сообщения
        for (_key, msg) in msgs.into_iter() {
            let msg = (self.fn_output)(msg);
            let Some(msg) = msg else { continue };
            self.output
                .send(msg)
                .await
                .map_err(|e| super::Error::TokioSyncMpsc(e.to_string()))?;
        }

        while let Some(msg) = self.input.recv().await {
            let msg = (self.fn_output)(msg);
            let Some(msg) = msg else { continue };

            self.output
                .send(msg)
                .await
                .map_err(|e| super::Error::TokioSyncMpsc(e.to_string()))?;
        }

        Err(super::Error::TaskEndOutput)
    }
}
