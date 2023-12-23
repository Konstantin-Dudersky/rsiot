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

use rsiot_component_core::{IComponent, Input, Output};
use rsiot_extra_components::{cmp_cache, cmp_mpsc_to_mpsc, cmpbase_mpsc_to_broadcast};
use rsiot_messages_core::IMessage;

use crate::{config::Config, errors::Errors};

use super::{async_task_utils::cancellable_task, handle_ws_connection::handle_ws_connection};

pub async fn process<TMessage>(
    input: Input<TMessage>,
    output: Output<TMessage>,
    config: Config<TMessage>,
) where
    TMessage: IMessage + 'static,
{
    info!("Component component_websocket_server started");

    let cancel = CancellationToken::new();
    let (cache_output, broadcast_input) = mpsc::channel::<TMessage>(1000);
    let (broadcast_output, mut _rx_broadcast) = broadcast::channel::<TMessage>(1000);
    let (stream_from_client, stream_to_output) = mpsc::channel::<TMessage>(1000);

    let cache = cmp_cache::create_cache::<TMessage>();

    // кэшируем данные
    let _task_cache = cmp_cache::new(cmp_cache::Config {
        cache: cache.clone(),
    })
    .set_and_spawn(input, Some(cache_output));

    let _task_to_output = cmp_mpsc_to_mpsc::create().set_and_spawn(Some(stream_to_output), output);

    // распространяем данные через broadcast
    let future = cmpbase_mpsc_to_broadcast::new(Some(broadcast_input), broadcast_output.clone());
    spawn(cancellable_task(future, cancel.clone()));

    loop {
        let result = task_main(
            cancel.clone(),
            broadcast_output.clone(),
            stream_from_client.clone(),
            cache.clone(),
            config.clone(),
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
    cancel: CancellationToken,
    input: broadcast::Sender<TMessage>,
    output: mpsc::Sender<TMessage>,
    cache: cmp_cache::CacheType<TMessage>,
    config: Config<TMessage>,
) -> Result<(), Errors>
where
    TMessage: IMessage + 'static,
{
    let addr = format!("0.0.0.0:{}", config.port);

    let listener = create_tcp_listener(addr).await?;

    // слушаем порт, при получении запроса создаем новое подключение WS
    while let Ok(stream_and_addr) = listener.accept().await {
        let future = handle_ws_connection(
            input.subscribe(),
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
