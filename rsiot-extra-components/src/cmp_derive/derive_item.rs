use super::derive_item_process::DeriveItemProcess;

pub struct DeriveItem<TMsg, TStore>
where
    TStore: Default + Send + Sync,
{
    /// Структура для сохранения промежуточных данных из входящих сообщений
    pub store: TStore,

    /// Обработка входящих сообщений и сохранение в `store`
    pub fn_input: fn(msg: &TMsg, store: &mut TStore) -> (),

    /// Формирование исходящих сообщений на основе данных, сохраненных в `store`
    pub fn_output: fn(store: &TStore) -> Option<Vec<TMsg>>,
}

impl<TMsg, TStore> DeriveItemProcess<TMsg> for DeriveItem<TMsg, TStore>
where
    TStore: Clone + Default + PartialEq + Send + Sync,
{
    fn process(&mut self, msg: &TMsg) -> Option<Vec<TMsg>> {
        let old_store = self.store.clone();
        (self.fn_input)(msg, &mut self.store);
        if old_store == self.store {
            return None;
        }
        (self.fn_output)(&self.store)
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
