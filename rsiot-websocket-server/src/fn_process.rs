//! Компонент для подключения через websocket server.
//!
//! Перенаправляет поток входящих сообщений подключенным вебсокет-клиентам
//!

use tokio::{
    net::TcpListener,
    spawn,
    time::{sleep, Duration},
};
use tokio_util::sync::CancellationToken;
use tracing::{error, info};

use rsiot_component_core::{Cache, CmpInput, CmpOutput, ComponentError};
use rsiot_messages_core::message_v2::MsgDataBound;

use crate::{config::Config, errors::Error};

use super::{async_task_utils::cancellable_task, handle_ws_connection::handle_ws_connection};

pub async fn fn_process<TMessage>(
    input: CmpInput<TMessage>,
    output: CmpOutput<TMessage>,
    config: Config<TMessage>,
    cache: Cache<TMessage>,
) -> Result<(), ComponentError>
where
    TMessage: MsgDataBound + 'static,
{
    info!(
        "Component cmp_websocket_server started. Config: {:?}",
        config
    );

    let cancel = CancellationToken::new();

    loop {
        let result = task_main(
            input.clone(),
            output.clone(),
            config.clone(),
            cache.clone(),
            cancel.clone(),
        )
        .await;
        match result {
            Ok(_) => (),
            Err(err) => error!("{:?}", err),
        }
        info!("Restarting...");
        sleep(Duration::from_secs(2)).await;
    }
}

async fn task_main<TMessage>(
    input: CmpInput<TMessage>,
    output: CmpOutput<TMessage>,
    config: Config<TMessage>,
    cache: Cache<TMessage>,
    cancel: CancellationToken,
) -> crate::Result<()>
where
    TMessage: MsgDataBound + 'static,
{
    let addr = format!("0.0.0.0:{}", config.port);

    let listener = create_tcp_listener(addr).await?;

    // слушаем порт, при получении запроса создаем новое подключение WS
    while let Ok(stream_and_addr) = listener.accept().await {
        let future = handle_ws_connection(
            input.clone(),
            output.clone(),
            config.clone(),
            stream_and_addr,
            cache.clone(),
        );
        spawn(cancellable_task(future, cancel.clone()));
    }

    Ok(())
}

async fn create_tcp_listener(addr: String) -> crate::Result<TcpListener> {
    let listener = TcpListener::bind(&addr).await;
    let listener = match listener {
        Ok(value) => value,
        Err(error) => {
            return Err(Error::BindToPort(error));
        }
    };
    info!("Listening on: {}", addr);
    Ok(listener)
}
