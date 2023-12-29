use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
};

use axum::routing;
use tokio::{
    spawn,
    time::{sleep, Duration},
};
use tower_http::{
    cors::CorsLayer,
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
    LatencyUnit,
};
use tracing::{error, info, Level};

use rsiot_component_core::{ComponentInput, ComponentOutput};
use rsiot_extra_components::cmpbase_cache;
use rsiot_messages_core::IMessage;

use crate::{config::Config, error::Error, routes, shared_state::SharedState};

/// Компонент для получения и ввода сообщений через HTTP Server
pub async fn fn_process<TMessage>(
    input: ComponentInput<TMessage>,
    output: ComponentOutput<TMessage>,
    config: Config<TMessage>,
) where
    TMessage: IMessage + 'static,
{
    // кэшируем данные
    let cache = cmpbase_cache::create_cache::<TMessage>();
    let _task_cache = spawn(cmpbase_cache::cmpbase_cache(
        input,
        cmpbase_cache::Config {
            cache: cache.clone(),
        },
    ));

    // общее состояние
    let shared_state = Arc::new(SharedState { cache, output });

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
    let ipaddr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
    let socket_addr = SocketAddr::new(ipaddr, port);

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

    let listener = tokio::net::TcpListener::bind(socket_addr)
        .await
        .map_err(|err| Error::BindPort(err))?;

    axum::serve(listener, app)
        .await
        .map_err(|err| Error::AxumServe(err))?;

    Ok(())
}