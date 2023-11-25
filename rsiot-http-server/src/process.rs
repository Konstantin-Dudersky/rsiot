use std::sync::Arc;

use axum::routing;
use tokio::time::{sleep, Duration};
use tower_http::{
    cors::CorsLayer,
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
    LatencyUnit,
};
use tracing::{error, info, Level};

use rsiot_component_core::{IComponent, StreamInput, StreamOutput};
use rsiot_extra_components::cmp_cache;
use rsiot_messages_core::IMessage;

use crate::{config::Config, error::Error, routes, shared_state::SharedState};

/// Компонент для получения и ввода сообщений через HTTP Server
pub async fn process<TMessage>(
    input: StreamInput<TMessage>,
    output: StreamOutput<TMessage>,
    config: Config,
) where
    TMessage: IMessage + 'static,
{
    // кэшируем данные
    let cache = cmp_cache::create_cache::<TMessage>();
    let _task_cache = cmp_cache::new(cmp_cache::Config {
        cache: cache.clone(),
    })
    .set_and_spawn(input, None);

    // общее состояние
    let shared_state = Arc::new(SharedState {
        cache,
        stream_output: output.unwrap(),
    });

    loop {
        info!("Component started");
        let result = task_main(shared_state.clone(), config.port).await;
        if let Err(err) = result {
            error!("{:?}", err);
        }
        info!("Restarting...");
        sleep(Duration::from_secs(2)).await
    }
}

async fn task_main<TMessage>(
    shared_state: Arc<SharedState<TMessage>>,
    port: u16,
) -> Result<(), Error<TMessage>>
where
    TMessage: IMessage + 'static,
{
    let url = format!("0.0.0.0:{}", port);
    let url = url.parse()?;

    let layer_cors = CorsLayer::permissive();

    let layer_trace = TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().level(Level::DEBUG))
        .on_request(DefaultOnRequest::new().level(Level::DEBUG))
        .on_response(
            DefaultOnResponse::new()
                .level(Level::DEBUG)
                .latency_unit(LatencyUnit::Micros),
        );

    let app = routing::Router::new()
        .route("/messages", routing::get(routes::list::<TMessage>))
        .route("/messages/:id", routing::get(routes::get::<TMessage>))
        .route("/messages", routing::put(routes::replace::<TMessage>))
        .with_state(shared_state)
        .layer(layer_cors)
        .layer(layer_trace);

    axum::Server::bind(&url)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
