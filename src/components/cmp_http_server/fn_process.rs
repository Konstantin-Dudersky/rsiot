use std::sync::Arc;

use axum::routing;
use tokio::{sync::Mutex, task::JoinSet};
use tower_http::{
    cors::CorsLayer,
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
    LatencyUnit,
};
use tracing::{info, Level};

use crate::{
    executor::{join_set_spawn, CmpInOut, ComponentError},
    message::MsgDataBound,
};

use super::{config::Config, routes, shared_state::SharedState, tasks};

/// Компонент для получения и ввода сообщений через HTTP Server
pub async fn fn_process<TMsg>(
    msg_bus: CmpInOut<TMsg>,
    config: Config<TMsg>,
) -> Result<(), ComponentError>
where
    TMsg: MsgDataBound + 'static,
{
    info!("Component started, configuration: {:?}", config);

    // Общее состояние
    let shared_state = Arc::new(Mutex::new(SharedState {
        msg_bus: msg_bus.clone(),
        config: config.clone(),
        cmp_plc_input: "Data not received".to_string(),
        cmp_plc_output: "Data not received".to_string(),
        cmp_plc_static: "Data not received".to_string(),
    }));

    let mut task_set: JoinSet<super::Result<()>> = JoinSet::new();

    // Задача обработки данных из `cmp_plc` --------------------------------------------------------
    let task = tasks::CmpPlcData {
        input: msg_bus.clone(),
        shared_state: shared_state.clone(),
        fn_input: config.cmp_plc,
    };
    join_set_spawn(&mut task_set, task.spawn());

    // Задача работы сервера Axum ------------------------------------------------------------------
    let layer_cors = CorsLayer::permissive();

    let layer_trace = TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().level(Level::DEBUG))
        .on_request(DefaultOnRequest::new().level(Level::DEBUG))
        .on_response(
            DefaultOnResponse::new()
                .level(Level::DEBUG)
                .latency_unit(LatencyUnit::Micros),
        );

    let router = routing::Router::new()
        .route("/", routing::get(routes::root))
        .route("/messages", routing::get(routes::list::<TMsg>))
        .route("/messages/:id", routing::get(routes::get::<TMsg>))
        .route("/messages", routing::put(routes::replace::<TMsg>))
        .route("/plc/input", routing::get(routes::plc_input::<TMsg>))
        .route("/plc/output", routing::get(routes::plc_output::<TMsg>))
        .route("/plc/static", routing::get(routes::plc_static::<TMsg>))
        .with_state(shared_state)
        .layer(layer_cors)
        .layer(layer_trace);

    let task = tasks::AxumServe {
        port: config.port,
        router,
    };
    join_set_spawn(&mut task_set, task.spawn());

    // Ждем выполнения всех задач ------------------------------------------------------------------
    while let Some(res) = task_set.join_next().await {
        res.unwrap().unwrap()
    }
    Ok(())
}
