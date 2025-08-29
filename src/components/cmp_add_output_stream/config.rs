use tokio::sync::mpsc;

use crate::message::Message;

/// Настройки компонента cmp_add_output_stream
#[derive(Debug)]
pub struct Config<TMessage> {
    /// Внешний канал mpsc, в который пересылаются исходящие сообщения
    pub channel: mpsc::Sender<Message<TMessage>>,
}
