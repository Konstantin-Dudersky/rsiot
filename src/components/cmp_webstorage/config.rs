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
    ///
    /// **Примеры**
    ///
    /// ```rust
    #[doc = include_str!("test/config_fn_input.rs")]
    /// ```
    pub fn_input: FnInput<TMsg>,

    /// Загрузка сообщений из хранилища
    ///
    /// **Примеры**
    ///
    /// ```rust
    #[doc = include_str!("test/config_fn_output.rs")]
    /// ```
    pub fn_output: FnOutput<TMsg>,

    /// Сообщения по-умолчанию, когда хранилище пустое
    ///
    /// **Примеры**
    ///
    /// ```rust
    #[doc = include_str!("test/config_default_messages.rs")]
    /// ```
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
#[derive(Clone, Copy, Debug)]
pub enum ConfigStorageKind {
    /// Сохраняет данные при перезапуске браузера
    LocalStorage,
    /// Сохраняет данные. При перезапуске браузера данные теряются
    SessionStorage,
}
