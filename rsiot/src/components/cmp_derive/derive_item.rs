use crate::message::{Message, MsgDataBound};

use super::derive_item_process::DeriveItemProcess;

/// Хранение данных для преобразования
pub struct DeriveItem<TMsg, TStore>
where
    TStore: Default + Send + Sync,
{
    /// Структура для сохранения промежуточных данных из входящих сообщений
    pub store: TStore,

    /// Обработка входящих сообщений и сохранение в `store`
    pub fn_input: fn(msg: &Message<TMsg>, store: &mut TStore) -> (),

    /// Формирование исходящих сообщений на основе данных, сохраненных в `store`
    pub fn_output: fn(store: &TStore) -> Option<Vec<Message<TMsg>>>,
}

impl<TMsg, TStore> DeriveItemProcess<TMsg> for DeriveItem<TMsg, TStore>
where
    TMsg: MsgDataBound,
    TStore: Clone + Default + PartialEq + Send + Sync,
{
    fn process(&mut self, msg: &Message<TMsg>) -> Option<Vec<Message<TMsg>>> {
        let old_store = self.store.clone();

        (self.fn_input)(msg, &mut self.store);
        if old_store == self.store {
            return None;
        }
        let msgs_content_data = (self.fn_output)(&self.store)?;
        Some(msgs_content_data)
    }
}
