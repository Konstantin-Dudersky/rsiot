use gloo::storage::{LocalStorage, SessionStorage, Storage};

use crate::message::MsgDataBound;

use super::{
    super::{config::FnInput, ConfigStorageKind},
    TaskInput, TaskOutput,
};

pub struct Input<TMsg>
where
    TMsg: MsgDataBound,
{
    pub input: TaskInput<TMsg>,
    pub output: TaskOutput<TMsg>,
    pub storage_kind: ConfigStorageKind,
    pub fn_input: FnInput<TMsg>,
}

impl<TMsg> Input<TMsg>
where
    TMsg: MsgDataBound,
{
    pub async fn spawn(mut self) -> super::Result<()> {
        while let Some(msg) = self.input.recv().await {
            let msg = (self.fn_input)(msg);
            let Some(msg) = msg else { continue };
            match self.storage_kind {
                ConfigStorageKind::LocalStorage => LocalStorage::set(msg.key.clone(), &msg)?,
                ConfigStorageKind::SessionStorage => SessionStorage::set(msg.key.clone(), &msg)?,
            };
            // Отправляем сообщение на выход
            self.output
                .send(msg)
                .await
                .map_err(|e| super::Error::TokioSyncMpsc(e.to_string()))?;
        }

        Err(super::Error::TaskEndInput)
    }
}
