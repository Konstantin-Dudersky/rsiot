use tokio::process::Command as TokioCommand;

use crate::{
    executor::{MsgBusInput, MsgBusOutput},
    message::MsgDataBound,
};

use super::{ConfigCommand, Error, ExecResult};

pub struct TaskCommand<TMsg>
where
    TMsg: MsgDataBound,
{
    pub msgbus_input: MsgBusInput<TMsg>,
    pub msgbus_output: MsgBusOutput<TMsg>,
    pub config: ConfigCommand<TMsg>,
}

impl<TMsg> TaskCommand<TMsg>
where
    TMsg: MsgDataBound,
{
    pub async fn spawn(mut self) -> Result<(), Error> {
        while let Ok(msg) = self.msgbus_input.recv().await {
            let Some(msg) = msg.get_custom_data() else {
                continue;
            };

            let cmds = (self.config.fn_input)(&msg);
            let Some(cmds) = cmds else { continue };

            let mut exec_results = vec![];

            for cmd in cmds {
                let exec_result = execute_command(&cmd).await?;
                exec_results.push(exec_result);
            }

            let msgs = (self.config.fn_output)(&exec_results);
            let Some(msgs) = msgs else { continue };

            for msg in msgs {
                self.msgbus_output
                    .send(msg.to_message())
                    .await
                    .map_err(|_| Error::TokioSyncMpscSend)?;
            }
        }

        Err(Error::TaskCommandEnd)
    }
}

async fn execute_command(cmd: &str) -> Result<ExecResult, Error> {
    let cmd = string_to_tokio_command(cmd)?.output();

    let output = match cmd.await {
        Ok(v) => v,
        Err(_) => todo!(),
    };

    let exec_result = ExecResult {
        status: output.status.to_string(),
        stdout: String::from_utf8(output.stdout)?,
        stderr: String::from_utf8(output.stderr)?,
    };

    Ok(exec_result)
}

fn string_to_tokio_command(input: &str) -> Result<TokioCommand, Error> {
    let parts = input.split(" ").collect::<Vec<&str>>();
    let program = parts.first().ok_or(Error::EmptyCommand)?;
    let mut cmd = TokioCommand::new(program);
    for arg in parts[1..].iter() {
        cmd.arg(arg);
    }
    Ok(cmd)
}
