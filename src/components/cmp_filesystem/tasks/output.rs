use tokio::fs::{read, read_dir};
use tracing::warn;

use crate::{executor::CmpInOut, message::MsgDataBound};

use super::super::{config::FnOutput, Error};

pub async fn output<TMsg>(
    directory: String,
    config_fn_output: FnOutput<TMsg>,
    in_out: CmpInOut<TMsg>,
) -> super::Result<()>
where
    TMsg: MsgDataBound,
{
    // Читаем содержимое папки
    let mut reader = read_dir(directory).await.map_err(Error::CreateDirError)?;
    while let Some(entry) = reader
        .next_entry()
        .await
        .map_err(Error::ReadDirEntryError)?
    {
        let content = read(entry.path()).await.map_err(Error::ReadFileError)?;
        let content = String::from_utf8_lossy(&content);
        let msg = (config_fn_output)(&content.to_string());
        let msg = match msg {
            Ok(ok) => ok,
            Err(err) => {
                let err = err.to_string();
                warn!("File not load. File: {:?}. Error: {}", entry.path(), err);
                return Ok(());
            }
        };
        let Some(msg) = msg else { return Ok(()) };
        in_out.send_output(msg).await.map_err(Error::CmpOutput)?;
    }

    Ok(())
}
