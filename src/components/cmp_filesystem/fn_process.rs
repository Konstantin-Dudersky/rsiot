use tokio::fs::{read, try_exists, write};
use tracing::warn;

use crate::{
    executor::{MsgBusInput, MsgBusOutput},
    message::{Message, MsgDataBound},
    serde_utils::SerdeAlg,
};

use super::{BufferBound, CallFnOutputKind, Config, Error};

pub async fn fn_process<TMsg, TBuffer>(
    config: Config<TMsg, TBuffer>,
    mut input: MsgBusInput<TMsg>,
    output: MsgBusOutput<TMsg>,
) -> super::Result<()>
where
    TMsg: MsgDataBound + 'static,
    TBuffer: BufferBound,
{
    let serde_alg = SerdeAlg::new(config.serde_alg);

    // Проверяем, существует ли файл
    let check_exist = try_exists(&config.filename).await;
    let need_file_create = match check_exist {
        Err(err) => {
            warn!("Read file error: {:?}", err);
            true
        }
        Ok(v) if !v => {
            warn!("File {} not found", config.filename);
            true
        }
        _ => false,
    };

    // Создаем файл при необходимости
    if need_file_create {
        let buffer = TBuffer::default();
        write_to_file(&config.filename, &serde_alg, &buffer).await?;
    }

    // Читаем файл
    let buffer = read_from_file(&config.filename, &serde_alg).await;
    let mut buffer: TBuffer = match buffer {
        Ok(buffer) => buffer,
        Err(err) => {
            warn!("Read file error: {:?}, maybe buffer struct changed", err);
            let buffer = TBuffer::default();
            write_to_file(&config.filename, &serde_alg, &buffer).await?;
            buffer
        }
    };

    send_messages(config.fn_output, &buffer, &output).await?;

    while let Ok(msg) = input.recv().await {
        let Some(msg) = msg.get_custom_data() else {
            continue;
        };
        let new_buffer = (config.fn_input)(&msg, &buffer);
        let Some(new_buffer) = new_buffer else {
            continue;
        };
        buffer = new_buffer;
        write_to_file(&config.filename, &serde_alg, &buffer).await?;
        if matches!(config.call_fn_output_kind, CallFnOutputKind::OnStartup) {
            continue;
        }
        buffer = read_from_file(&config.filename, &serde_alg).await?;
        send_messages(config.fn_output, &buffer, &output).await?;
    }

    Ok(())
}

async fn write_to_file<TBuffer>(
    filename: &str,
    serde_alg: &SerdeAlg,
    buffer: &TBuffer,
) -> super::Result<()>
where
    TBuffer: BufferBound,
{
    let buffer_bytes = serde_alg.serialize(buffer)?;
    write(filename, buffer_bytes)
        .await
        .map_err(|e| Error::WriteFileError(e, filename.to_string()))?;
    Ok(())
}

async fn read_from_file<TBuffer>(filename: &str, serde_alg: &SerdeAlg) -> super::Result<TBuffer>
where
    TBuffer: BufferBound,
{
    let buffer_bytes = read(filename).await.map_err(super::Error::ReadFileError)?;
    let buffer: TBuffer = serde_alg.deserialize(&buffer_bytes)?;
    Ok(buffer)
}

async fn send_messages<TMsg, TBuffer>(
    fn_output: super::config::FnOutput<TMsg, TBuffer>,
    buffer: &TBuffer,
    msgbus_output: &MsgBusOutput<TMsg>,
) -> super::Result<()>
where
    TMsg: MsgDataBound,
    TBuffer: BufferBound,
{
    let msgs = (fn_output)(buffer);
    for msg in msgs {
        let msg = Message::new_custom(msg);
        msgbus_output
            .send(msg)
            .await
            .map_err(|_| super::Error::TokioMpscSend)?;
    }
    Ok(())
}
