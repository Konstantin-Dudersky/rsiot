use std::{sync::Arc, time::Duration};

use instant::Instant;
use serde::Serialize;
use tokio::sync::{mpsc, Mutex};
use tracing::{info, trace};

use crate::{
    executor::{sleep, Cache},
    message::{Message, MsgDataBound},
};

use super::super::{
    plc::{FunctionBlockBase, IFunctionBlock},
    Config,
};

/// Задача выполнения цикла ПЛК
pub struct PlcLoop<TMsg, I, Q, S>
where
    TMsg: MsgDataBound + 'static,
    I: Clone + Default + Send + Serialize + Sync,
    Q: Clone + Default + Send + Serialize + Sync,
    S: Clone + Default + Send + Serialize + Sync,
    FunctionBlockBase<I, Q, S>: IFunctionBlock<I, Q, S>,
{
    pub input_msg_cache: Cache<TMsg>,
    pub output: mpsc::Sender<Message<TMsg>>,
    pub config: Config<TMsg, I, Q, S>,
    pub fb_main: Arc<Mutex<FunctionBlockBase<I, Q, S>>>,
}

impl<TMsg, I, Q, S> PlcLoop<TMsg, I, Q, S>
where
    TMsg: MsgDataBound + 'static,
    I: Clone + Default + Send + Serialize + Sync,
    Q: Clone + Default + Send + Serialize + Sync,
    S: Clone + Default + Send + Serialize + Sync,
    FunctionBlockBase<I, Q, S>: IFunctionBlock<I, Q, S>,
{
    pub async fn spawn(self) -> super::Result<()> {
        info!("PLC mode: STARTED");
        let mut fb_main_input = I::default();

        loop {
            trace!("Start PLC cycle");
            let begin = Instant::now();

            // Исполняем цикл ПЛК
            let msgs = plc_cycle::<TMsg, I, Q, S>(
                &self.config,
                self.fb_main.clone(),
                &mut fb_main_input,
                self.input_msg_cache.clone(),
            )
            .await?;

            // Записываем выходы
            for msg in msgs {
                self.output
                    .send(msg)
                    .await
                    .map_err(|e| super::Error::TokioSyncMpsc(e.to_string()))?;
            }

            let elapsed = begin.elapsed();
            trace!("End PLC cycle, elapsed: {:?}", elapsed);
            let sleep_time = if self.config.period <= elapsed {
                Duration::from_millis(10)
            } else {
                self.config.period - elapsed
            };
            sleep(sleep_time).await;
        }
    }
}

/// Исполнение одного цикла ПЛК
async fn plc_cycle<TMsg, I, Q, S>(
    config: &Config<TMsg, I, Q, S>,
    fb_main: Arc<Mutex<FunctionBlockBase<I, Q, S>>>,
    fb_main_input: &mut I,
    input_msg_cache: Cache<TMsg>,
) -> super::Result<Vec<Message<TMsg>>>
where
    TMsg: MsgDataBound + 'static,
    I: Clone + Default + Send + Serialize,
    Q: Clone + Default + Send + Serialize,
    S: Clone + Default + Send + Serialize,
    FunctionBlockBase<I, Q, S>: IFunctionBlock<I, Q, S>,
{
    // Инициализация структуры входов в начале цикла
    (config.fn_cycle_init)(fb_main_input);

    // Обновляем входную структуру по данным из входящих сообщений
    {
        let mut lock = input_msg_cache.write().await;
        for msg in lock.values() {
            (config.fn_input)(fb_main_input, msg);
        }
        lock.clear();
    }

    // Выполняем цикл ПЛК и формируем исходящие сообщения
    let msgs;
    {
        let mut fb_main = fb_main.lock().await;
        fb_main.call(fb_main_input, config.period);
        msgs = (config.fn_output)(&fb_main.output);
    }
    Ok(msgs)
}
