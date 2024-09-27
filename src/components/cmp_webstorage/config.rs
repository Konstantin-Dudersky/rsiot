use crate::message::*;

pub type FnInput<TMsg> = fn(Message<TMsg>) -> Option<Message<TMsg>>;

pub type FnOutput<TMsg> = fn(Message<TMsg>) -> Option<Message<TMsg>>;

/// Конфигуреция cmp_webstorage
pub struct Config<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Вид хранилища
    pub storage_kind: ConfigStorageKind,

    /// Сохранение сообщений в хранилище
    pub fn_input: FnInput<TMsg>,

    /// Загрузка сообщений из хранилища
    pub fn_output: FnOutput<TMsg>,

    /// Сообщения по-умолчанию, когда хранилище пустое
    pub default_messages: Vec<Message<TMsg>>,
}

impl<TMsg> Default for Config<TMsg>
where
    TMsg: MsgDataBound,
{
    fn default() -> Self {
        Self {
            storage_kind: ConfigStorageKind::LocalStorage,
            fn_input: |_| None,
            fn_output: |_| None,
            default_messages: vec![],
        }
    }
}

/// Вид хранилища - localstorage или sessionStorage
#[derive(Clone, Copy)]
pub enum ConfigStorageKind {
    /// Сохраняет данные при перезапуске браузера
    LocalStorage,
    /// Сохраняет данные. При перезапуске браузера данные теряются
    SessionStorage,
}

#[cfg(test)]
mod tests {
    use crate::{components::cmp_webstorage, message::example_message::*};

    use super::Message;

    #[test]
    fn fn_input() {
        // Сохраняем все сообщения
        let fn_input_0 = |msg: Message<Custom>| Some(msg.clone());
        // Не сохранять ничего
        let fn_input_1 = |_| None;

        let _ = cmp_webstorage::Config::<Custom> {
            fn_input: fn_input_0,
            ..Default::default()
        };
        let _ = cmp_webstorage::Config::<Custom> {
            fn_input: fn_input_1,
            ..Default::default()
        };
    }

    #[test]
    fn fn_output() {
        // Ничего не загружать
        let fn_output_0 = |_| None;
        // Загружать все сообщения
        let fn_output_1 = |msg: Message<Custom>| Some(msg.clone());

        let _ = cmp_webstorage::Config::<Custom> {
            fn_output: fn_output_0,
            ..Default::default()
        };
        let _ = cmp_webstorage::Config::<Custom> {
            fn_output: fn_output_1,
            ..Default::default()
        };
    }

    #[test]
    fn default_messages() {
        // Пустой массив
        let default_messages_0 = vec![];
        // Есть значения
        let default_messages_1 = vec![Message::new_custom(Custom::ValueInstantF64(1.2))];

        let _ = cmp_webstorage::Config::<Custom> {
            default_messages: default_messages_0,
            ..Default::default()
        };
        let _ = cmp_webstorage::Config::<Custom> {
            default_messages: default_messages_1,
            ..Default::default()
        };
    }
}
