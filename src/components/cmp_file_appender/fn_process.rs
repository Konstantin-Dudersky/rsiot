use tokio::{fs::File, io::AsyncWriteExt};

use crate::{executor::MsgBusLinker, message::MsgDataBound};

use super::{Config, ConfigAction, Error};

pub async fn fn_process<TMsg>(
    config: Config<TMsg>,
    msgbus_linker: MsgBusLinker<TMsg>,
) -> super::Result<()>
where
    TMsg: MsgDataBound,
{
    let mut input = msgbus_linker.input();
    msgbus_linker.close();

    let mut file = File::create(config.filename)
        .await
        .map_err(Error::CreateFile)?;

    while let Ok(msg) = input.recv().await {
        let Some(msg) = msg.get_custom_data() else {
            continue;
        };

        let action = (config.fn_input)(&msg);

        match action {
            ConfigAction::NoAction => continue,
            ConfigAction::AppendLine(line) => {
                let line = line + "\n";
                file.write_all(line.as_bytes())
                    .await
                    .map_err(Error::WriteAllFile)?;
            }
            ConfigAction::EndProcessing => {
                file.flush().await.map_err(Error::FlushFile)?;
                break;
            }
        };
    }

    Ok(())
}
