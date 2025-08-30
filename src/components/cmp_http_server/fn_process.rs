use std::sync::Arc;

use axum::routing;
use tokio::{sync::Mutex, task::JoinSet};
use tower_http::{
    LatencyUnit,
    cors::CorsLayer,
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};
use tracing::{Level, info};

use crate::{
    components_config::http_server::{GetEndpointsCollection, PutEndpointsCollection},
    executor::{CmpInOut, join_set_spawn},
    message::MsgDataBound,
};

use super::{config::Config, routes, shared_state::SharedState, tasks};

/// Компонент для получения и ввода сообщений через HTTP Server
pub async fn fn_process<TMsg>(msg_bus: CmpInOut<TMsg>, config: Config<TMsg>) -> super::Result<()>
where
    TMsg: MsgDataBound + 'static,
{
    info!("Component started, configuration: {:?}", config);

    let get_endpoints = GetEndpointsCollection::new(&config.get_endpoints);
    let get_endpoints_paths = get_endpoints.all_paths();
    let get_endpoints = Arc::new(Mutex::new(get_endpoints));

    let put_endpoints = PutEndpointsCollection::new(&config.put_endpoints);
    let put_endpoints_paths = put_endpoints.all_paths();
    let put_endpoints = Arc::new(Mutex::new(put_endpoints));

    // Общее состояние
    let shared_state = SharedState {
        msg_bus: msg_bus.clone(),
        get_endpoints: get_endpoints.clone(),
        put_endpoints: put_endpoints.clone(),
    };

    let mut task_set: JoinSet<super::Result<()>> = JoinSet::new();

    // Задача обновления данных точек GET ----------------------------------------------------------
    let task = tasks::UpdateGetEndpoints {
        input: msg_bus.clone(),
        get_endpoints: get_endpoints.clone(),
    };
    join_set_spawn(
        &mut task_set,
        "cmp_http_server | update_get_endpoints",
        task.spawn(),
    );

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

    let mut router = routing::Router::new();

    // Добавляем обработчики для GET запросов
    for path in get_endpoints_paths {
        router = router.route(&path, routing::get(routes::get));
    }

    // Добавляем обработчики для PUT запросов
    for path in put_endpoints_paths {
        router = router.route(&path, routing::put(routes::put));
    }

    let router = router
        .route("/", routing::get(routes::root))
        .with_state(shared_state)
        .layer(layer_cors)
        .layer(layer_trace);

    let task = tasks::AxumServe {
        port: config.port,
        router,
    };
    join_set_spawn(&mut task_set, "cmp_http_server | axum_serve", task.spawn());

    // Ждем выполнения всех задач ------------------------------------------------------------------
    while let Some(res) = task_set.join_next().await {
        res??
    }
    Ok(())
}
