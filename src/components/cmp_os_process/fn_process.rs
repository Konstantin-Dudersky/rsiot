use std::time::Duration;

use tokio::process::Command;
use tokio::time::sleep;

use crate::{executor::CmpInOut, message::MsgDataBound};

use super::Config;

pub async fn fn_process<TMsg>(_config: Config<TMsg>, _msg_bus: CmpInOut<TMsg>) -> super::Result<()>
where
    TMsg: MsgDataBound,
{
    loop {
        let output = Command::new("echo").arg("hello").arg("world").output();

        let output = output.await.unwrap();

        println!("Status: {}", output.status.success());
        println!("Stdout: {:?}", output.stdout);

        sleep(Duration::from_secs(2)).await;
    }
}
