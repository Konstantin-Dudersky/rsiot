use std::time::Duration;
use time::OffsetDateTime;
use tokio::sync::{broadcast, mpsc};

use super::{EmaKind, Error, IntMsgBound, OutputValue, Result};

pub struct Task<TIntMsg>
where
    TIntMsg: IntMsgBound,
{
    pub input: broadcast::Receiver<TIntMsg>,
    pub output: mpsc::Sender<TIntMsg>,
    pub fn_input_value: fn(TIntMsg) -> Option<(f64, OffsetDateTime)>,
    pub fn_input_time_window: fn(TIntMsg) -> Option<Duration>,
    pub fn_output: fn(OutputValue) -> TIntMsg,
    pub kind: EmaKind,
}

impl<TIntMsg> Task<TIntMsg>
where
    TIntMsg: IntMsgBound,
{
    pub async fn spawn(mut self) -> Result<()> {
        let mut time_window = Duration::default();
        let mut prev_value: f64 = 0.0;
        let mut prev_time = OffsetDateTime::now_utc();
        let mut prev_ema = 0.0;
        let mut init = true;

        while let Ok(input_int_msg) = self.input.recv().await {
            // Получаем новое значение окна времени
            if let Some(new_time_window) = (self.fn_input_time_window)(input_int_msg) {
                time_window = new_time_window;
            }

            // Получаем новое значение
            if let Some((value, time)) = (self.fn_input_value)(input_int_msg) {
                // Пропускаем первое значение
                if init {
                    prev_value = value;
                    prev_time = time;
                    init = false;
                    continue;
                }

                let ema = EmaCalc {
                    kind: self.kind,
                    prev_time,
                    prev_value,
                    prev_ema,
                    value,
                    time,
                    time_window,
                }
                .calc();
                prev_value = value;
                prev_time = time;
                prev_ema = ema;

                let out_value = OutputValue { ema, time };
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

struct EmaCalc {
    pub kind: EmaKind,
    pub prev_time: OffsetDateTime,
    pub prev_value: f64,
    pub prev_ema: f64,
    pub value: f64,
    pub time: OffsetDateTime,
    pub time_window: Duration,
}
impl EmaCalc {
    pub fn calc(self) -> f64 {
        match self.kind {
            EmaKind::Last => self.last(),
            EmaKind::Next => self.next(),
            EmaKind::Linear => self.linear(),
        }
    }

    fn last(self) -> f64 {
        let w = (-(self.time - self.prev_time) / self.time_window).exp();
        self.prev_ema * w + self.prev_value * (1.0 - w)
    }

    fn next(self) -> f64 {
        let w = (-(self.time - self.prev_time) / self.time_window).exp();
        self.prev_ema * w + self.value * (1.0 - w)
    }

    fn linear(self) -> f64 {
        let tmp = (self.time - self.prev_time) / self.time_window;
        let w = (-tmp).exp();
        let w2 = (1.0 - w) / tmp;
        self.prev_ema * w + self.value * (1.0 - w2) + self.prev_value * (w2 - w)
    }
}
