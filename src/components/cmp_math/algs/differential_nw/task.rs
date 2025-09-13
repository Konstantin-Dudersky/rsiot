use std::time::Duration;
use time::OffsetDateTime;
use tokio::sync::{broadcast, mpsc};

use super::{Error, IntMsgBound, OutputValue, Result};

const BETA: f64 = 0.65;
const GAMMA: f64 = 1.22208;
const ALPHA: f64 = 1.0 / (GAMMA * (8.0 * BETA - 3.0));

pub struct Task<TIntMsg>
where
    TIntMsg: IntMsgBound,
{
    pub input: broadcast::Receiver<TIntMsg>,
    pub output: mpsc::Sender<TIntMsg>,
    pub fn_input_value: fn(TIntMsg) -> Option<(f64, OffsetDateTime)>,
    pub fn_input_time_window: fn(TIntMsg) -> Option<Duration>,
    pub fn_output: fn(OutputValue) -> TIntMsg,
}

impl<TIntMsg> Task<TIntMsg>
where
    TIntMsg: IntMsgBound,
{
    pub async fn spawn(mut self) -> Result<()> {
        let mut time_window = Duration::default();
        let mut state = State::default();

        while let Ok(input_int_msg) = self.input.recv().await {
            // Получаем новое значение окна времени
            if let Some(new_time_window) = (self.fn_input_time_window)(input_int_msg) {
                time_window = new_time_window;
            }

            // Получаем новое значение
            if let Some((value, time)) = (self.fn_input_value)(input_int_msg) {
                let out_value = state.step(value, time, time_window);

                let output_int_msg = (self.fn_output)(out_value);
                self.output
                    .send(output_int_msg)
                    .await
                    .map_err(|_| Error::AlgTaskUnexpectedEnd(String::from("EMA")))?;
            };
        }

        let err = String::from("EMA");
        Err(Error::AlgTaskUnexpectedEnd(err))
    }
}

#[derive(Default)]
enum State {
    /// Начало выполнения
    #[default]
    Init,

    /// Получили первое значение
    FirstValue {
        prev_value: f64,
        prev_time: OffsetDateTime,
    },

    /// Стандартное состояние
    Normal {
        prev_ema_1: [f64; 3],
        prev_ema_2: [f64; 5],
        prev_time: OffsetDateTime,
    },
}
impl State {
    pub fn step(
        &mut self,
        new_value: f64,
        new_time: OffsetDateTime,
        time_window: Duration,
    ) -> OutputValue {
        match self {
            State::Init => {
                *self = Self::FirstValue {
                    prev_value: new_value,
                    prev_time: new_time,
                };
                OutputValue {
                    differential: 0.0,
                    time: new_time,
                    time_window,
                }
            }
            State::FirstValue {
                prev_value,
                prev_time,
            } => {
                let ema1 = repeat_linear_ema::<3>(
                    &[*prev_value, 0.0, 0.0],
                    *prev_time,
                    new_value,
                    new_time,
                    mul_duration(time_window, ALPHA),
                );
                let ema2 = repeat_linear_ema::<5>(
                    &[*prev_value, 0.0, 0.0, 0.0, 0.0],
                    *prev_time,
                    new_value,
                    new_time,
                    mul_duration(time_window, ALPHA * BETA),
                );

                let differential = GAMMA * (ema1[1] + ema1[2] - 2.0 * ema2[4]);

                *self = Self::Normal {
                    prev_ema_1: ema1,
                    prev_ema_2: ema2,
                    prev_time: *prev_time,
                };
                OutputValue {
                    differential,
                    time: new_time,
                    time_window,
                }
            }
            State::Normal {
                prev_ema_1,
                prev_ema_2,
                prev_time,
            } => {
                let new_ema_1 = repeat_linear_ema::<3>(
                    prev_ema_1,
                    *prev_time,
                    new_value,
                    new_time,
                    mul_duration(time_window, ALPHA),
                );
                println!("{new_ema_1:?}");
                let new_ema_2 = repeat_linear_ema::<5>(
                    prev_ema_2,
                    *prev_time,
                    new_value,
                    new_time,
                    mul_duration(time_window, ALPHA * BETA),
                );
                println!("{new_ema_2:?}");

                let differential = GAMMA * (new_ema_1[1] + new_ema_1[2] - 2.0 * new_ema_2[4]);
                *self = Self::Normal {
                    prev_ema_1: new_ema_1,
                    prev_ema_2: new_ema_2,
                    prev_time: *prev_time,
                };
                OutputValue {
                    differential,
                    time: new_time,
                    time_window,
                }
            }
        }
    }
}

fn repeat_linear_ema<const ORDER_PLUS_ONE: usize>(
    prev_ema: &[f64; ORDER_PLUS_ONE],
    prev_time: OffsetDateTime,
    new_value: f64,
    new_time: OffsetDateTime,
    time_window: Duration,
) -> [f64; ORDER_PLUS_ONE] {
    let tmp = (new_time - prev_time) / time_window;
    let w = (-tmp).exp();
    let w2 = (1.0 - w) / tmp;

    let mut new_ema = [0.0; ORDER_PLUS_ONE];
    new_ema[0] = new_value;

    for order in 1..ORDER_PLUS_ONE {
        new_ema[order] =
            prev_ema[order] * w + new_ema[order - 1] * (1.0 - w2) + prev_ema[order - 1] * (w2 - w);
    }

    new_ema
}

fn mul_duration(duration: Duration, float: f64) -> Duration {
    let duration_nanos = duration.as_nanos() as f64;
    let new_duration = duration_nanos * float;
    Duration::from_nanos(new_duration as u64)
}
