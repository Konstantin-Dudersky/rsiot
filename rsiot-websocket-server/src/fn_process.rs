//! Компонент для подключения через websocket server.
//!
//! Перенаправляет поток входящих сообщений подключенным вебсокет-клиентам
//!

use tokio::{
    net::TcpListener,
    spawn,
    sync::{broadcast, mpsc},
    time::{sleep, Duration},
};
use tokio_util::sync::CancellationToken;
use tracing::{error, info};

use rsiot_component_core::{Cache, ComponentError, ComponentInput, ComponentOutput};
use rsiot_messages_core::IMessage;

use crate::{config::Config, errors::Errors};

use super::{async_task_utils::cancellable_task, handle_ws_connection::handle_ws_connection};

pub async fn fn_process<TMessage>(
    input: ComponentInput<TMessage>,
    output: ComponentOutput<TMessage>,
    config: Config<TMessage>,
    cache: Cache<TMessage>,
) -> Result<(), ComponentError>
where
    TMessage: IMessage + 'static,
{
    info!("Component component_websocket_server started");

    let cancel = CancellationToken::new();

    loop {
        let result = task_main(
            input.resubscribe(),
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
    input: broadcast::Receiver<TMessage>,
    output: mpsc::Sender<TMessage>,
    config: Config<TMessage>,
    cache: Cache<TMessage>,
    cancel: CancellationToken,
) -> Result<(), Errors>
where
    TMessage: IMessage + 'static,
{
    let addr = format!("0.0.0.0:{}", config.port);

    let listener = create_tcp_listener(addr).await?;

    // слушаем порт, при получении запроса создаем новое подключение WS
    while let Ok(stream_and_addr) = listener.accept().await {
        let future = handle_ws_connection(
            input.resubscribe(),
            output.clone(),
            config.clone(),
            stream_and_addr,
            cache.clone(),
        );
        spawn(cancellable_task(future, cancel.clone()));
    }

    Ok(())
}

async fn create_tcp_listener(addr: String) -> Result<TcpListener, Errors> {
    let listener = TcpListener::bind(&addr).await;
    let listener = match listener {
        Ok(value) => value,
        Err(error) => {
            return Err(Errors::BindToPort(error));
        }
    };
    info!("Listening on: {}", addr);
    Ok(listener)
}
