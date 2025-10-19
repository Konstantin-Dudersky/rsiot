use std::{collections::HashMap, path::Path};

use tokio::{
    fs::{File, create_dir_all},
    io::AsyncWriteExt,
};
use tracing::warn;

use crate::{executor::MsgBusLinker, message::MsgDataBound};

use super::{Config, ConfigAction, Error};

pub async fn fn_process<TMsg, TFnInput>(
    config: Config<TMsg, TFnInput>,
    msgbus_linker: MsgBusLinker<TMsg>,
) -> super::Result<()>
where
    TMsg: MsgDataBound,
    TFnInput: Fn(TMsg) -> ConfigAction + Send + Sync,
{
    let mut input = msgbus_linker.input();
    msgbus_linker.close();

    let mut files = HashMap::new();

    while let Ok(msg) = input.recv().await {
        let Some(msg) = msg.get_custom_data() else {
            continue;
        };

        let action = (config.fn_input)(msg);

        match action {
            ConfigAction::NoAction => continue,
            ConfigAction::AppendLine { filename, line } => {
                action_append_line(&mut files, filename, line).await?;
            }
            ConfigAction::EndProcessing => {
                action_end_processing(&mut files).await?;
                break;
            }
        };
    }

    Ok(())
}

async fn action_append_line(
    files: &mut HashMap<String, File>,
    filename: String,
    line: String,
) -> Result<(), Error> {
    if !files.contains_key(&filename) {
        // Заменяем символы, не допустимые в имени файла Windows
        let filename_save = filename.replace(":", "\u{A789}");

        let path = Path::new(&filename_save);

        // Создаём все папки, если необходимо
        if let Some(parent) = path.parent() {
            create_dir_all(parent).await.map_err(Error::CreateDirAll)?;
        }

        // Создаём файл
        let file = File::create(path).await.map_err(Error::CreateFile)?;

        files.insert(filename.clone(), file);
    }

    let file = files.get_mut(&filename);
    let Some(file) = file else {
        warn!("File not found in file collection: {}", filename);
        return Ok(());
    };

    let line = line + "\n";
    file.write_all(line.as_bytes())
        .await
        .map_err(Error::WriteAllFile)?;

    Ok(())
}

async fn action_end_processing(files: &mut HashMap<String, File>) -> Result<(), Error> {
    for file in files.values_mut() {
        file.flush().await.map_err(Error::FlushFile)?;
    }
    Ok(())
}
