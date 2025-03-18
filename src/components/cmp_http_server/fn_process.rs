use std::{collections::HashMap, sync::Arc};

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
    message::{MsgDataBound, ServiceBound},
};

use super::{
    config::Config, routes, shared_state::SharedState, tasks, GetEndpoint, GetEndpointsHashMap,
    PutEndpoint, PutEndpointsHashMap,
};

/// Компонент для получения и ввода сообщений через HTTP Server
pub async fn fn_process<TMsg, TService>(
    msg_bus: CmpInOut<TMsg, TService>,
    config: Config<TMsg>,
) -> Result<(), ComponentError>
where
    TMsg: MsgDataBound + 'static,
    TService: ServiceBound + 'static,
{
    info!("Component started, configuration: {:?}", config);

    let get_endpoints = create_get_endpoints_hashmap(&config.get_endpoints);
    let get_endpoints_paths = get_endpoints
        .keys()
        .map(|k| k.to_string())
        .collect::<Vec<String>>();
    let get_endpoints = Arc::new(Mutex::new(get_endpoints));

    let put_endpoints = create_put_endpoints_hashmap(&config.put_endpoints);
    let put_endpoints_paths = put_endpoints
        .keys()
        .map(|k| k.to_string())
        .collect::<Vec<String>>();
    let put_endpoints = Arc::new(Mutex::new(put_endpoints));

    // Общее состояние
    let shared_state = Arc::new(Mutex::new(SharedState {
        msg_bus: msg_bus.clone(),
        config: config.clone(),
        cmp_plc_input: "Data not received".to_string(),
        cmp_plc_output: "Data not received".to_string(),
        cmp_plc_static: "Data not received".to_string(),
        get_endpoints: get_endpoints.clone(),
        put_endpoints: put_endpoints.clone(),
    }));

    let mut task_set: JoinSet<super::Result<()>> = JoinSet::new();

    // Задача обновления данных точек GET ----------------------------------------------------------
    let task = tasks::UpdateGetEndpoints {
        input: msg_bus.clone(),
        get_endpoints: get_endpoints.clone(),
    };
    join_set_spawn(&mut task_set, task.spawn());

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

    let mut router = routing::Router::new();

    // Добавляем обработчики для GET запросов
    for path in get_endpoints_paths {
        router = router.route(&path, routing::get(routes::get_new));
    }

    // Добавляем обработчики для PUT запросов
    for path in put_endpoints_paths {
        router = router.route(&path, routing::put(routes::put));
    }

    let router = router
        // .route("/", routing::get(routes::root))
        // .route("/messages", routing::get(routes::list))
        // .route("/messages/{id}", routing::get(routes::get))
        // .route("/messages", routing::put(routes::replace))
        // .route("/plc/input", routing::get(routes::plc_input))
        // .route("/plc/output", routing::get(routes::plc_output))
        // .route("/plc/static", routing::get(routes::plc_static))
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

fn create_get_endpoints_hashmap<TMsg>(
    config_endpoints: &[Box<dyn GetEndpoint<TMsg>>],
) -> GetEndpointsHashMap<TMsg>
where
    TMsg: MsgDataBound,
{
    let mut endpoints = HashMap::new();
    for endpoint in config_endpoints {
        endpoints.insert(endpoint.get_path().to_string(), endpoint.clone());
    }
    endpoints
}

fn create_put_endpoints_hashmap<TMsg>(
    config_endpoints: &[Box<dyn PutEndpoint<TMsg>>],
) -> PutEndpointsHashMap<TMsg>
where
    TMsg: MsgDataBound,
{
    let mut endpoints = HashMap::new();
    for endpoint in config_endpoints {
        endpoints.insert(endpoint.get_path().to_string(), endpoint.clone());
    }
    endpoints
}
