use rsiot_messages_core::{Message, MsgDataBound, MsgType};

use super::derive_item_process::DeriveItemProcess;

pub struct DeriveItem<TMsg, TStore>
where
    TStore: Default + Send + Sync,
{
    /// Структура для сохранения промежуточных данных из входящих сообщений
    pub store: TStore,

    /// Обработка входящих сообщений и сохранение в `store`
    pub fn_input: fn(msg_content_data: &TMsg, store: &mut TStore) -> (),

    /// Формирование исходящих сообщений на основе данных, сохраненных в `store`
    pub fn_output: fn(store: &TStore) -> Option<Vec<TMsg>>,
}

impl<TMsg, TStore> DeriveItemProcess<TMsg> for DeriveItem<TMsg, TStore>
where
    TMsg: MsgDataBound,
    TStore: Clone + Default + PartialEq + Send + Sync,
{
    fn process(&mut self, msg: &Message<TMsg>) -> Option<Vec<Message<TMsg>>> {
        let old_store = self.store.clone();
        let msg_content_data = match &msg.data {
            MsgType::System(_) => return None,
            MsgType::Custom(msg_data) => msg_data,
        };
        (self.fn_input)(msg_content_data, &mut self.store);
        if old_store == self.store {
            return None;
        }
        let msgs_content_data = (self.fn_output)(&self.store)?;
        let msgs_vec = msgs_content_data
            .into_iter()
            .map(Message::new_custom)
            .collect();
        Some(msgs_vec)
    }
}

// TODO - не компилируется
// impl<TMsg, TStore> From<DeriveItem<TMsg, TStore>> for Box<DeriveItem<TMsg, TStore>>
// where
//     TStore: Default,
// {
//     fn from(value: DeriveItem<TMsg, TStore>) -> Self {
//         Box::new(value)
//     }
// }
