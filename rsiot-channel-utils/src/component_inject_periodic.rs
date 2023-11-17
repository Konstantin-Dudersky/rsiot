use tokio::{
    sync::mpsc,
    time::{sleep, Duration, Instant},
};

use rsiot_messages_core::IMessage;

/// Компонент для периодического генерирования сообщений
pub async fn component_inject_periodic<TMessage, TFnPeriodic>(
    stream_output: mpsc::Sender<TMessage>,
    period: Duration,
    mut fn_periodic: TFnPeriodic,
) where
    TMessage: IMessage,
    TFnPeriodic: FnMut() -> Vec<TMessage>,
{
    loop {
        let begin = Instant::now();
        let msgs = (fn_periodic)();
        for msg in msgs {
            stream_output.send(msg).await.unwrap();
        }
        let time_to_sleep = period - begin.elapsed();
        sleep(time_to_sleep).await;
    }
}
