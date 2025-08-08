use std::time::Duration;

use slint::ComponentHandle;
use tokio::sync::mpsc;

use crate::message::MsgDataBound;

pub type FnInput<TMsg, TMainWindow> = fn(TMsg, TMainWindow);
pub type FnOutput<TMsg, TMainWindow> = fn(TMainWindow, mpsc::Sender<TMsg>);

// ANCHOR: Config
/// Настройки компонента cmp_slint
pub struct Config<TMsg, TMainWindow>
where
    Self: Sync,
    TMsg: MsgDataBound,
    TMainWindow: ComponentHandle,
{
    /// Ссылка на главное окно
    pub slint_window: super::SlintWindow<TMainWindow>,

    /// Функция обработки входящих сообщений
    ///
    /// *Пример:*
    ///
    /// ```rust
    /// fn_input: |msg, w| {
    ///     let input_data = w.global::<Input>();
    ///     let Some(msg) = msg.get_custom_data() else {
    ///         return;
    ///     };
    ///     match msg {
    ///         Custom::LiveCounter(msg) => match msg {
    ///             Livecounter::Counter(c) => input_data.set_value_from_phone(c as i32),
    ///         },
    ///         _ => (),
    ///     };
    /// },
    /// ```
    pub fn_input: FnInput<TMsg, TMainWindow>,

    /// Функция генерирования исходящих сообщений
    ///
    /// *Пример:*
    ///
    /// ```rust
    /// fn_output: |w, tx| {
    ///     let output_data = w.global::<Output>();
    ///     output_data.on_slider(move |value| {
    ///         let msg = Message::new_custom(Custom::Slint(Slint::Slider(value as f64)));
    ///         tx.blocking_send(msg).unwrap();
    ///     })
    /// },
    /// ```
    pub fn_output: FnOutput<TMsg, TMainWindow>,

    /// Период фильтрации исходящих сообщений
    pub output_period: Duration,
}
// ANCHOR: Config

impl<TMsg, TMainWindow> Clone for Config<TMsg, TMainWindow>
where
    TMsg: MsgDataBound,
    TMainWindow: ComponentHandle,
{
    fn clone(&self) -> Self {
        Self {
            slint_window: self.slint_window.clone(),
            fn_input: self.fn_input,
            fn_output: self.fn_output,
            output_period: Duration::from_millis(100),
        }
    }
}
