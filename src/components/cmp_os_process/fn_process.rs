use std::time::Duration;

use tokio::process::Command;
use tokio::time::sleep;

use crate::{executor::CmpInOut, message::MsgDataBound};

use super::Config;

pub async fn fn_process<TMsg>(
    _config: Config<TMsg>,
    msgbus_linker: CmpInOut<TMsg>,
) -> super::Result<()>
where
    TMsg: MsgDataBound,
{
    msgbus_linker.close();
    loop {
        let output = Command::new("echo").arg("hello").arg("world").output();

        let output = match output.await {
            Ok(v) => v,
            Err(_) => todo!(),
        };

        println!("Status: {}", output.status.success());
        println!("Stdout: {:?}", output.stdout);

        sleep(Duration::from_secs(2)).await;
    }
}
