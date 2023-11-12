use std::sync::Arc;

use axum::routing;
use tokio::{
    spawn,
    sync::mpsc,
    time::{sleep, Duration},
};
use tower_http::cors::CorsLayer;

use rsiot_channel_utils::{component_cache, create_cache};
use rsiot_messages_core::IMessage;
use tracing::{error, info};

use crate::{
    error::Error, route_message_get::route_message_get,
    route_message_put::route_message_put, shared_state::SharedState,
};

/// Компонент для получения и ввода сообщений через HTTP Server
pub async fn component_http_server<TMessage>(
    stream_input: mpsc::Receiver<TMessage>,
    stream_output: mpsc::Sender<TMessage>,
    port: u16,
) -> ()
where
    TMessage: IMessage + 'static,
{
    // кеширование входящих сообщений
    let cache = create_cache::<TMessage>();
    let _task_cache = spawn(component_cache(stream_input, None, cache.clone()));

    // общее состояние
    let shared_state = Arc::new(SharedState {
        cache,
        stream_output,
    });

    loop {
        info!("Component started");
        let result = loop_(shared_state.clone(), port).await;
        if let Err(err) = result {
            error!("{:?}", err);
        }
        info!("Restarting...");
        sleep(Duration::from_secs(2)).await
    }
}

async fn loop_<TMessage>(
    shared_state: Arc<SharedState<TMessage>>,
    port: u16,
) -> Result<(), Error<TMessage>>
where
    TMessage: IMessage + 'static,
{
    let url = format!("0.0.0.0:{}", port);
    let url = url.parse()?;

    let app = routing::Router::new()
        .route("/message/:id", routing::get(route_message_get::<TMessage>))
        .route("/message", routing::put(route_message_put::<TMessage>))
        .with_state(shared_state)
        .layer(CorsLayer::permissive());

    axum::Server::bind(&url)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
