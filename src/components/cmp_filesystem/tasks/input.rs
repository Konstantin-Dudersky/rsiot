use tokio::fs::write;

use crate::{executor::CmpInOut, message::MsgDataBound};

use super::super::{config::FnInput, Error};

pub async fn input<TMsg>(
    directory: String,
    config_fn_input: FnInput<TMsg>,
    mut in_out: CmpInOut<TMsg>,
) -> super::Result<()>
where
    TMsg: MsgDataBound,
{
    while let Ok(msg) = in_out.recv_input().await {
        let name_and_content = (config_fn_input)(msg).map_err(Error::FnInput)?;
        let Some((name, content)) = name_and_content else {
            continue;
        };
        let name = format!("{directory}/{name}");
        write(&name, content).await.map_err(Error::WriteFileError)?;
    }
    Ok(())
}
