use std::collections::VecDeque;

use std::time::Duration;
use time::OffsetDateTime;

use crate::{
    executor::MsgBusOutput,
    message::{MsgDataBound, ValueTime},
};

use super::{AlgFnOutputMsgbus, AlgInput, AlgOutput, Error, OutputValue};

pub struct Task<TMsg>
where
    TMsg: MsgDataBound,
{
    pub input: AlgInput,
    pub output: AlgOutput,
    pub output_msgbus: MsgBusOutput<TMsg>,
    pub time_window: Duration,
    pub fn_output_msgbus: AlgFnOutputMsgbus<TMsg, OutputValue>,
}

impl<TMsg> Task<TMsg>
where
    TMsg: MsgDataBound,
{
    pub async fn spawn(mut self) -> Result<(), Error> {
        let mut buffer: VecDeque<ValueInBuffer> = VecDeque::new();
        let mut out_value: Option<ValueInBuffer> = None;

        while let Some(input_value) = self.input.recv().await {
            let buffer_back = match buffer.back() {
                // Если в буфере есть значение, рассчитываем площадь
                Some(last) => {
                    let area = calc_area(last.time, input_value.time, last.value);
                    ValueInBuffer {
                        value: input_value.value,
                        time: input_value.time,
                        area,
                    }
                }
                None => ValueInBuffer {
                    value: input_value.value,
                    time: input_value.time,
                    area: 0.0,
                },
            };

            // Добавляем новое значение в конец буфера
            buffer.push_back(buffer_back);

            // Удаляем значения, метка времени которых не попадает в окно
            let begin_ts = buffer_back.time - self.time_window;
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
                    buffer_back.time - self.time_window,
                    out_value.value,
                );
                full_area -= left_area;
            }

            let sma = full_area / self.time_window.as_nanos() as f64;

            let out_value = OutputValue {
                sma,
                time: input_value.time,
            };

            let msg = (self.fn_output_msgbus)(&out_value);
            if let Some(msg) = msg {
                self.output_msgbus
                    .send(msg.to_message())
                    .await
                    .map_err(|_| Error::SendToMsgbus)?;
            }

            self.output
                .send(ValueTime {
                    value: out_value.sma,
                    time: out_value.time,
                })
                .await
                .map_err(|_| Error::AlgTaskUnexpectedEnd(String::from("SMA")))?;
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
