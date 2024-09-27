use std::collections::HashMap;

use gloo::storage::{LocalStorage, SessionStorage, Storage};

use crate::message::{Message, MsgDataBound};

use super::{
    super::{config::FnOutput, ConfigStorageKind},
    TaskOutput,
};

pub struct Output<TMsg> {
    pub output: TaskOutput<TMsg>,
    pub storage_kind: ConfigStorageKind,
    pub default_messages: Vec<Message<TMsg>>,
    pub fn_output: FnOutput<TMsg>,
}

impl<TMsg> Output<TMsg>
where
    TMsg: MsgDataBound,
{
    pub async fn spawn(self) -> super::Result<()> {
        // Загружаем из хранилища все значения
        let mut msgs: HashMap<String, Message<TMsg>> = match self.storage_kind {
            ConfigStorageKind::LocalStorage => LocalStorage::get_all()?,
            ConfigStorageKind::SessionStorage => SessionStorage::get_all()?,
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
        Ok(())
    }
}
