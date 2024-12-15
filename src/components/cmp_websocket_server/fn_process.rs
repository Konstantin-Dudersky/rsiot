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

use crate::{
    executor::{CmpInOut, ComponentError},
    message::{AuthPermissions, MsgDataBound, ServiceBound},
};

use super::{
    async_task_utils::cancellable_task, config::Config, errors::Error,
    handle_ws_connection::handle_ws_connection,
};

pub async fn fn_process<TMessage, TService>(
    input: CmpInOut<TMessage, TService>,
    config: Config<TMessage>,
) -> Result<(), ComponentError>
where
    TMessage: MsgDataBound + 'static,
    TService: ServiceBound + 'static,
{
    info!(
        "Component cmp_websocket_server started. Config: {:?}",
        config
    );

    let cancel = CancellationToken::new();

    loop {
        let result = task_main(input.clone(), config.clone(), cancel.clone()).await;
        match result {
            Ok(_) => (),
            Err(err) => error!("{:?}", err),
        }
        info!("Restarting...");
        sleep(Duration::from_secs(2)).await;
    }
}

async fn task_main<TMessage, TService>(
    in_out: CmpInOut<TMessage, TService>,
    config: Config<TMessage>,
    cancel: CancellationToken,
) -> super::Result<()>
where
    TMessage: MsgDataBound + 'static,
    TService: ServiceBound + 'static,
{
    let addr = format!("0.0.0.0:{}", config.port);

    let listener = create_tcp_listener(addr).await?;

    // слушаем порт, при получении запроса создаем новое подключение WS
    while let Ok(stream_and_addr) = listener.accept().await {
        let session_name = format!("session_{}", stream_and_addr.1);
        let future = handle_ws_connection(
            in_out.clone_with_new_id(&session_name, AuthPermissions::FullAccess),
            config.clone(),
            stream_and_addr,
        );
        spawn(cancellable_task(future, cancel.clone()));
    }

    Ok(())
}

async fn create_tcp_listener(addr: String) -> super::Result<TcpListener> {
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
