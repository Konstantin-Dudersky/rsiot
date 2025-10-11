use std::time::Duration;

use tokio::sync::{broadcast, mpsc};
use tracing::{debug, info, trace, warn};

use crate::message::{Message, MsgData, MsgDataBound, system_messages::System};

use super::{Cache, ComponentError, LessInPeriod, sleep};

/// Уровень переполненности канала. Чем ближе к 1.0, тем раньше появится сообщение переполнения
const CHANNEL_FULL: f64 = 0.3;

pub struct TaskInternal<TMsg>
where
    TMsg: MsgDataBound,
{
    pub input: broadcast::Sender<Message<TMsg>>,
    pub output: mpsc::Receiver<Message<TMsg>>,
    pub cache: Cache<TMsg>,
    pub delay_publish: Duration,
    pub max_capacity: usize,
}
impl<TMsg> TaskInternal<TMsg>
where
    TMsg: MsgDataBound,
{
    pub async fn spawn(mut self) -> Result<(), ComponentError> {
        debug!("Internal task of ComponentExecutor: starting");

        // Задержка, чтобы компоненты успели запуститься и подписаться на получение сообщений
        sleep(self.delay_publish).await;

        let stat = format!(
            r#"
MsgBus statistics:
Connected to input:           {}
Connected to output (strong): {}
Channels capacity:            {}
"#,
            self.input.receiver_count(),
            self.output.sender_strong_count(),
            self.max_capacity
        );

        info!("{stat}");

        let max_capacity = self.max_capacity as f64;

        let mut input_less_in_period = LessInPeriod::new(Duration::from_millis(500));
        let mut output_less_in_period = LessInPeriod::new(Duration::from_millis(500));

        while let Some(msg) = self.output.recv().await {
            trace!("ComponentExecutor: new message: {:?}", msg);
            let msg = save_msg_in_cache(msg, &self.cache).await;
            let Some(msg) = msg else { continue };

            // Проверяем переполненность канала
            check_input_overflow(&self.input, max_capacity, &mut input_less_in_period)?;
            check_output_overflow(
                &self.input,
                &self.output,
                max_capacity,
                &mut output_less_in_period,
            )?;

            self.input
                .send(msg)
                .map_err(|_| ComponentError::TaskInternalSend)?;
        }
        warn!("Internal task: stop");
        Ok(())
    }
}

/// Сохраняем сообщение в кеше
///
/// Возвращает `Option<Message>`:
/// - None - сообщение не нужно отправлять дальше
/// - Some(Message) - сообщение нужно отправить на вход всех компонентов
async fn save_msg_in_cache<TMsg>(msg: Message<TMsg>, cache: &Cache<TMsg>) -> Option<Message<TMsg>>
where
    TMsg: MsgDataBound,
{
    // Фильтруем сообщения авторизации
    if let MsgData::System(data) = &msg.data {
        match data {
            System::InputChannelFull => return Some(msg),
            System::OutputChannelFull => return Some(msg),
            System::AuthRequestByLogin(_) => return Some(msg),
            System::AuthRequestByToken(_) => return Some(msg),
            System::AuthResponseErr(_) => return Some(msg),
            System::AuthResponseOk(_) => return Some(msg),
            System::Ping(_) => return None,
            System::Pong(_) => return None,
        }
    }

    let key = msg.key.clone();
    let value = msg.clone();
    {
        let mut lock = cache.write().await;
        lock.insert(key, value);
    }
    Some(msg)
}

/// Функция проверки переполненности канала
fn check_input_overflow<TMsg>(
    input: &broadcast::Sender<Message<TMsg>>,
    buffer_size: f64,
    less_in_period: &mut LessInPeriod,
) -> Result<(), ComponentError>
where
    TMsg: MsgDataBound,
{
    let capacity = input.len() as f64;
    let percent = 1.0 - capacity / buffer_size;

    if percent <= CHANNEL_FULL && less_in_period.check() {
        warn!(
            "MsgBus input buffer is full; current free space: {}",
            percent * 100.0
        );
        let msg = Message::new(MsgData::System(System::InputChannelFull));
        input
            .send(msg)
            .map_err(|_| ComponentError::TaskInternalSend)?;
    }

    Ok(())
}

fn check_output_overflow<TMsg>(
    input: &broadcast::Sender<Message<TMsg>>,
    output: &mpsc::Receiver<Message<TMsg>>,
    max_capacity: f64,
    less_in_period: &mut LessInPeriod,
) -> Result<(), ComponentError>
where
    TMsg: MsgDataBound,
{
    let capacity = output.capacity() as f64;
    let percent = capacity / max_capacity;

    if percent <= CHANNEL_FULL && less_in_period.check() {
        warn!(
            "MsgBus output buffer is full; current free space: {}",
            percent * 100.0
        );
        let msg = Message::new(MsgData::System(System::OutputChannelFull));
        input
            .send(msg)
            .map_err(|_| ComponentError::TaskInternalSend)?;
    }

    Ok(())
}
