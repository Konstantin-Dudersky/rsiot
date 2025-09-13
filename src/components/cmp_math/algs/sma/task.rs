use std::collections::VecDeque;

use std::time::Duration;
use time::OffsetDateTime;
use tokio::sync::{broadcast, mpsc};

use super::{Error, IntMsgBound, OutputValue, Result};

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
        let mut buffer: VecDeque<ValueInBuffer> = VecDeque::new();
        let mut out_value: Option<ValueInBuffer> = None;

        while let Ok(input_int_msg) = self.input.recv().await {
            // Получаем новое значение окна времени
            if let Some(new_time_window) = (self.fn_input_time_window)(input_int_msg) {
                time_window = new_time_window;
            }

            // Получаем новое значение
            if let Some((value, time)) = (self.fn_input_value)(input_int_msg) {
                let buffer_back = match buffer.back() {
                    // Если в буфере есть значение, рассчитываем площадь
                    Some(last) => {
                        let area = calc_area(last.time, time, last.value);
                        ValueInBuffer { value, time, area }
                    }
                    None => ValueInBuffer {
                        value,
                        time,
                        area: 0.0,
                    },
                };

                // Добавляем новое значение в конец буфера
                buffer.push_back(buffer_back);

                // Удаляем значения, метка времени которых не попадает в окно
                let begin_ts = buffer_back.time - time_window;
                loop {
                    let value = buffer.front();
                    let Some(value) = value else { break };
                    if value.time < begin_ts {
                        out_value = buffer.pop_front();
                    } else {
                        break;
                    }
                }

                if buffer.is_empty() {
                    continue;
                }

                // Вычисляем полную площадь, которая может содержать "лишнюю" площадь
                let mut full_area = buffer.iter().map(|v| v.area).sum::<f64>();

                // Вычисляем "лишнюю" площадь, которая не попадает в окно
                if let Some(out_value) = out_value {
                    let left_area = calc_area(
                        out_value.time,
                        buffer_back.time - time_window,
                        out_value.value,
                    );
                    full_area -= left_area;
                }

                let sma = full_area / time_window.as_nanos() as f64;

                let out_value = OutputValue { sma, time };
                let output_int_msg = (self.fn_output)(out_value);
                self.output
                    .send(output_int_msg)
                    .await
                    .map_err(|_| Error::AlgTaskUnexpectedEnd(String::from("SMA")))?;
            };
        }

        let err = String::from("SMA");
        Err(Error::AlgTaskUnexpectedEnd(err))
    }
}

#[derive(Clone, Copy)]
struct ValueInBuffer {
    pub value: f64,
    pub time: OffsetDateTime,
    pub area: f64,
}

fn calc_area(x1: OffsetDateTime, x2: OffsetDateTime, y: f64) -> f64 {
    (x2 - x1).whole_nanoseconds() as f64 * y
}
