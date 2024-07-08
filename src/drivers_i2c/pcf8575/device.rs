use std::sync::Arc;

use tokio::{sync::Mutex, task::JoinSet};
use tracing::warn;

use crate::{executor::CmpInOut, message::MsgDataBound};

use super::{
    super::{I2cSlaveAddress, RsiotI2cDriverBase},
    state::State,
    task_read_inputs::TaskReadInputs,
    task_write_output::TaskWriteOutput,
    PCF8575PinMode, TPinFnOutput,
};

pub struct PCF8575<TMsg>
where
    TMsg: MsgDataBound,
{
    pub address: I2cSlaveAddress,
    pub pins: Vec<PCF8575PinMode<TMsg>>,
}

impl<TMsg> PCF8575<TMsg>
where
    TMsg: MsgDataBound + 'static,
{
    pub async fn fn_process(
        &self,
        in_out: CmpInOut<TMsg>,
        driver: Arc<Mutex<impl RsiotI2cDriverBase + Send + 'static>>,
    ) {
        loop {
            let mut state = State::new();
            let mut task_set: JoinSet<Result<(), String>> = JoinSet::new();

            // Определяем начальную конфигурацию входов / выходов
            let mut input_pins: TPinFnOutput<TMsg> = vec![];
            for (index, pin) in self.pins.iter().enumerate() {
                match pin {
                    PCF8575PinMode::Disabled => {}

                    PCF8575PinMode::Input { fn_output } => {
                        state.set_input(index).await;
                        input_pins.push((index, *fn_output))
                    }

                    PCF8575PinMode::Output { fn_input } => {
                        state.set_output_low(index).await;
                        let mut task_output = TaskWriteOutput {
                            in_out: in_out.clone(),
                            fn_input: *fn_input,
                            state: state.clone(),
                            driver: driver.clone(),
                            address: self.address,
                            pin: index,
                        };
                        task_set.spawn(async move { task_output.spawn().await });
                    }
                }
            }

            let task_input = TaskReadInputs {
                in_out: in_out.clone(),
                driver: driver.clone(),
                address: self.address,
                pin_and_fn_output: input_pins,
                state: state.clone(),
            };
            task_set.spawn(async move { task_input.spawn().await });

            // задачи не должны заканчиваться. Если закончилась хоть одна - отменяем все остальные и
            // запускам все сначала
            while let Some(res) = task_set.join_next().await {
                warn!("{res:?}");
                task_set.abort_all()
            }
        }
    }
}
