use std::time::Duration;

use tokio::process::Command;
use tokio::time::sleep;

use crate::{
    executor::CmpInOut,
    message::{MsgDataBound, ServiceBound},
};

use super::Config;

pub async fn fn_process<TMsg, TService>(
    _config: Config<TMsg>,
    _msg_bus: CmpInOut<TMsg, TService>,
) -> super::Result<()>
where
    TMsg: MsgDataBound,
    TService: ServiceBound,
{
    loop {
        let output = Command::new("echo").arg("hello").arg("world").output();

        let output = output.await.unwrap();

        println!("Status: {}", output.status.success());
        println!("Stdout: {:?}", output.stdout);

        sleep(Duration::from_secs(2)).await;
    }
}
