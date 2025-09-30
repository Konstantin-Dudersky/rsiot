use crate::message::*;

// ANCHOR: Config
pub type FnInput<TMsg> = fn(Message<TMsg>) -> Option<Message<TMsg>>;

pub type FnOutput<TMsg> = fn(Message<TMsg>) -> Option<Message<TMsg>>;

/// Конфигурация cmp_webstorage
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
// ANCHOR: Config

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

// ANCHOR: ConfigStorageKind
/// Вид хранилища - localstorage или sessionStorage
#[derive(Clone, Copy, Debug)]
pub enum ConfigStorageKind {
    /// Сохраняет данные при перезапуске браузера
    LocalStorage,
    /// Сохраняет данные. При перезапуске браузера данные теряются
    SessionStorage,
}
// ANCHOR: ConfigStorageKind
