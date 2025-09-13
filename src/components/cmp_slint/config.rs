use std::time::Duration;

use slint::ComponentHandle;
use tokio::sync::mpsc;
use tracing::warn;

use crate::message::MsgDataBound;

pub type FnInput<TMsg, TMainWindow> = fn(TMsg, TMainWindow);
pub type FnOutput<TMsg, TMainWindow> = fn(TMainWindow, OutputSender<TMsg>);

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

#[derive(Clone)]
pub struct OutputSender<TMsg>
where
    TMsg: MsgDataBound,
{
    tx: mpsc::Sender<TMsg>,
}
impl<TMsg> OutputSender<TMsg>
where
    TMsg: MsgDataBound,
{
    pub fn new(tx: &mpsc::Sender<TMsg>) -> Self {
        Self { tx: tx.clone() }
    }

    pub fn send(&self, msg: TMsg) {
        let res = self.tx.blocking_send(msg);

        if let Err(e) = res {
            warn!("Error sending from slint callback: {e:?}");
        }
    }
}
