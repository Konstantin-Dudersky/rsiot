use std::sync::Arc;

use slint::{ComponentHandle, Weak};
use tokio::sync::{mpsc, Mutex, MutexGuard};

use crate::message::{Message, MsgDataBound};

/// Настройки компонента cmp_slint
pub struct Config<TMainWindow, TMsg>
where
    Self: Sync,
    TMsg: MsgDataBound,
    TMainWindow: ComponentHandle,
{
    /// Ссылка на главное окно
    pub instance: Arc<Mutex<Weak<TMainWindow>>>,

    /// Функция обработки входящих сообщений
    pub fn_input: fn(Message<TMsg>, MutexGuard<Weak<TMainWindow>>) -> (),

    /// Функция генерирования исходящих сообщений
    pub fn_output: fn(MutexGuard<Weak<TMainWindow>>, mpsc::Sender<Message<TMsg>>),
}

impl<TMainWindow, TMsg> Clone for Config<TMainWindow, TMsg>
where
    TMsg: MsgDataBound,
    TMainWindow: ComponentHandle,
{
    fn clone(&self) -> Self {
        Self {
            instance: self.instance.clone(),
            fn_input: self.fn_input,
            fn_output: self.fn_output,
        }
    }
}
