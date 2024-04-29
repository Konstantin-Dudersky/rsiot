use std::{sync::Arc, time::Duration};

use bitvec::prelude::*;
use tokio::{sync::Mutex, task::JoinSet, time::sleep};
use tracing::warn;

use crate::{
    executor::CmpInOut,
    message::{Message, MsgDataBound},
};

use super::{super::RsiotI2cDriverBase, state::State, PCF8575PinMode};

pub struct PCF8575<TMsg>
where
    TMsg: MsgDataBound,
{
    pub address: u8,
    pub pins: Vec<PCF8575PinMode<TMsg>>,
}

impl<TMsg> PCF8575<TMsg>
where
    TMsg: MsgDataBound + 'static,
{
    pub async fn fn_process(
        &self,
        in_out: CmpInOut<TMsg>,
        driver: Arc<Mutex<impl RsiotI2cDriverBase + std::marker::Send + 'static>>,
    ) {
        loop {
            let mut state = State::new();
            let mut task_set: JoinSet<Result<(), String>> = JoinSet::new();

            // Определяем начальную конфигурацию входов / выходов
            let mut pin_and_fn_output: Vec<(usize, fn(bool) -> Option<Message<TMsg>>)> = vec![];
            for (index, pin) in self.pins.iter().enumerate() {
                match pin {
                    PCF8575PinMode::Input { fn_output } => {
                        state.set_input(index);
                        pin_and_fn_output.push((index, *fn_output))
                    }
                    PCF8575PinMode::Output { fn_input } => {
                        state.set_output_low(index);
                        task_set.spawn(task_output(
                            in_out,
                            *fn_input,
                            state,
                            driver.clone(),
                            self.address,
                            index,
                        ));
                    }
                    PCF8575PinMode::Disabled => {}
                }
            }

            let state = Arc::new(Mutex::new(state));

            task_set.spawn(task_input(
                in_out.clone(),
                driver.clone(),
                self.address,
                pin_and_fn_output,
                state.clone(),
            ));

            // задачи не должны заканчиваться. Если закончилась хоть одна - отменяем все остальные и
            // запускам все сначала
            while let Some(res) = task_set.join_next().await {
                warn!("{res:?}");
                task_set.abort_all()
            }
        }
    }
}

/// Чтение и обработка входов
async fn task_input<TMsg>(
    in_out: CmpInOut<TMsg>,
    driver: Arc<Mutex<impl RsiotI2cDriverBase>>,
    address: u8,
    pin_and_fn_output: Vec<(usize, fn(bool) -> Option<Message<TMsg>>)>,
    state: Arc<Mutex<State>>,
) -> Result<(), String>
where
    TMsg: MsgDataBound,
{
    let mut status_saved = 0u16;
    let status_saved_bits = status_saved.view_bits_mut::<Lsb0>();
    let mut first_cycle = true;

    loop {
        let state = {
            let state = state.lock().await;
            state.to_bytes()
        };
        let status_current = {
            let mut driver = driver.lock().await;
            driver
                .write_read(address, &state, 2)
                .await
                .map_err(String::from)?
        };
        let status_current_bits = status_current.view_bits::<Lsb0>();
        for (pin, fn_output) in &pin_and_fn_output {
            if (status_current_bits[*pin] != status_saved_bits[*pin]) || first_cycle {
                let msg = fn_output(!status_current_bits[*pin]);
                status_saved_bits.set(*pin, status_current_bits[*pin]);
                let Some(msg) = msg else { continue };
                in_out.send_output(msg).await.map_err(|e| e.to_string())?;
            }
        }
        first_cycle = false;

        sleep(Duration::from_millis(100)).await;
    }
}

/// Обработка и запись выходов
async fn task_output<TMsg>(
    mut in_out: CmpInOut<TMsg>,
    fn_input: fn(Message<TMsg>) -> Option<bool>,
    state: Arc<Mutex<State>>,
    driver: Arc<Mutex<impl RsiotI2cDriverBase>>,
    address: u8,
    pin: usize,
) -> Result<(), String>
where
    TMsg: MsgDataBound,
{
    while let Ok(msg) = in_out.recv_input().await {
        let value = fn_input(msg);
        let Some(value) = value else { continue };
        let state_bytes = {
            let mut state = state.lock().await;
            match value {
                true => state.set_output_high(pin),
                false => state.set_output_low(pin),
            }
            state.to_bytes()
        };
        {
            let mut driver = driver.lock().await;
            driver.write_read(address, &state_bytes, 2).await?;
        }
    }
    Ok(())
}
