use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
};

use axum::routing;
use tokio::time::{sleep, Duration};
use tower_http::{
    cors::CorsLayer,
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
    LatencyUnit,
};
use tracing::{error, info, Level};

use rsiot_component_core::{Cache, CmpOutput, ComponentError};
use rsiot_messages_core::MsgDataBound;

use crate::{config::Config, error::Error, routes, shared_state::SharedState};

/// Компонент для получения и ввода сообщений через HTTP Server
pub async fn fn_process<TMsg>(
    output: CmpOutput<TMsg>,
    config: Config<TMsg>,
    cache: Cache<TMsg>,
) -> Result<(), ComponentError>
where
    TMsg: MsgDataBound + 'static,
{
    info!("Component started, configuration: {:?}", config);
    // общее состояние
    let shared_state = Arc::new(SharedState {
        cache,
        output,
        config: config.clone(),
    });
    loop {
        let result = task_main(shared_state.clone(), config.port).await;
        if let Err(err) = result {
            error!("{:?}", err);
        }
        info!("Restarting...");
        sleep(Duration::from_secs(2)).await
    }
}

async fn task_main<TMsg>(shared_state: Arc<SharedState<TMsg>>, port: u16) -> Result<(), Error>
where
    TMsg: MsgDataBound + 'static,
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
        .route("/", routing::get(routes::root))
        .route("/messages", routing::get(routes::list::<TMsg>))
        .route("/messages/:id", routing::get(routes::get::<TMsg>))
        .route("/messages", routing::put(routes::replace::<TMsg>))
        .with_state(shared_state)
        .layer(layer_cors)
        .layer(layer_trace);

    let listener = tokio::net::TcpListener::bind(socket_addr)
        .await
        .map_err(Error::BindPort)?;

    axum::serve(listener, app).await.map_err(Error::AxumServe)?;

    Ok(())
}
