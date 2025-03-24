use std::{sync::Arc, time::Duration};

use embedded_svc::{
    http::{Headers, Method},
    io::{Read, Write},
};
use esp_idf_svc::http::server::{
    Configuration as HttpServerConfiguration, EspHttpConnection, EspHttpServer, Request,
};
use tokio::{sync::Mutex, task::JoinSet, time::sleep};
use tracing::{info, trace, warn};

use crate::{
    components_config::http_server::{
        create_get_endpoints_hashmap, create_put_endpoints_hashmap, handler_get, handler_put,
    },
    executor::{join_set_spawn, CmpInOut},
    message::{system_messages, MsgData, MsgDataBound, ServiceBound},
};

use super::{
    config::{handler_info, GetEndpointsHashMap, PutEndpointsHashMap},
    tasks, Config,
};

/// Заголовки для разрешения CORS
const HEADERS: [(&str, &str); 4] = [
    ("Access-Control-Allow-Origin", "*"),
    ("Access-Control-Max-Age", "600"),
    ("Access-Control-Allow-Methods", "PUT,POST,GET,OPTIONS"),
    ("Access-Control-Allow-Headers", "*"),
];

pub async fn fn_process<TMsg, TService>(
    mut in_out: CmpInOut<TMsg, TService>,
    config: Config<TMsg>,
) -> super::Result<()>
where
    TMsg: MsgDataBound + 'static,
    TService: ServiceBound + 'static,
{
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

    // Необходимо подождать, пока поднимется Wi-Fi
    while let Ok(msg) = in_out.recv_input().await {
        match msg.data {
            MsgData::System(system_messages::System::EspWifiConnected) => break,
            _ => continue,
        }
    }
    info!("Starting cmp_esp_http_server");

    let http_config = HttpServerConfiguration {
        http_port: config.port,
        ..Default::default()
    };

    let mut server = loop {
        info!("trying to create EspHttpServer");
        let server = EspHttpServer::new(&http_config);
        match server {
            Ok(server) => break server,
            Err(err) => {
                let err = format! {"Error EspHttpServer creation: {}", err};
                warn!("{}", err);
            }
        }
        sleep(Duration::from_secs(2)).await;
    };

    let mut task_set = JoinSet::new();

    let task = tasks::UpdateGetEndpoints {
        input: in_out.clone(),
        get_endpoints: get_endpoints.clone(),
    };
    join_set_spawn(&mut task_set, task.spawn());

    // Корневой запрос - перечень точек
    {
        let get_endpoints = get_endpoints.clone();
        let put_endpoints = put_endpoints.clone();
        server
            .fn_handler("/", Method::Get, move |request| {
                route_root(request, get_endpoints.clone(), put_endpoints.clone())
            })
            .map_err(super::Error::RegisterHandler)?;
    }

    // Запросы GET
    for path in get_endpoints_paths {
        let get_endpoints = get_endpoints.clone();
        server
            .fn_handler(&path, Method::Get, move |request| {
                route_get(request, get_endpoints.clone())
            })
            .map_err(super::Error::RegisterHandler)?;
    }

    // Запросы PUT
    for path in put_endpoints_paths.clone() {
        let put_endpoints = put_endpoints.clone();
        let msg_bus = in_out.clone();
        server
            .fn_handler(&path, Method::Put, move |request| {
                route_put(request, put_endpoints.clone(), msg_bus.clone())
            })
            .map_err(super::Error::RegisterHandler)?;
    }

    // Запросы POST
    for path in put_endpoints_paths {
        let put_endpoints = put_endpoints.clone();
        let msg_bus = in_out.clone();
        server
            .fn_handler(&path, Method::Post, move |request| {
                route_put(request, put_endpoints.clone(), msg_bus.clone())
            })
            .map_err(super::Error::RegisterHandler)?;
    }

    // Ждем выполнения всех задач ------------------------------------------------------------------
    while let Some(res) = task_set.join_next().await {
        res??
    }
    Ok(())
}

fn route_root<TMsg>(
    request: Request<&mut EspHttpConnection>,
    get_endpoints: Arc<Mutex<GetEndpointsHashMap<TMsg>>>,
    put_endpoints: Arc<Mutex<PutEndpointsHashMap<TMsg>>>,
) -> super::Result<()>
where
    TMsg: MsgDataBound,
{
    let body = handler_info(
        &get_endpoints.blocking_lock(),
        &put_endpoints.blocking_lock(),
    );

    send_response(request, 200, &body)?;
    Ok(())
}

fn route_get<TMsg>(
    request: Request<&mut EspHttpConnection>,
    get_endpoints: Arc<Mutex<GetEndpointsHashMap<TMsg>>>,
) -> super::Result<()>
where
    TMsg: MsgDataBound,
{
    let path = request.uri();
    trace!("Get request, path: {}", path);

    let response_body = handler_get(
        path,
        &get_endpoints.blocking_lock(),
        super::Error::UnknownPath,
        super::Error::SerdeJson,
    )?;

    send_response(request, 200, &response_body)?;
    Ok(())
}

fn route_put<TMsg, TService>(
    mut request: Request<&mut EspHttpConnection>,
    put_endpoints: Arc<Mutex<PutEndpointsHashMap<TMsg>>>,
    msg_bus: CmpInOut<TMsg, TService>,
) -> super::Result<()>
where
    TMsg: MsgDataBound,
    TService: ServiceBound,
{
    let path = &request.uri().to_string();
    trace!("Put request, path: {}", path);

    let body = read_request_body(&mut request)?;

    let msg = handler_put(
        path,
        &body,
        &put_endpoints.blocking_lock(),
        super::Error::UnknownPath,
        super::Error::SerdeJson,
    );
    let msg = match msg {
        Ok(val) => val,
        Err(err) => {
            send_response(request, 400, &err.to_string())?;
            return Err(err);
        }
    };

    let Some(msg) = msg else { return Ok(()) };

    msg_bus.send_output_blocking(msg)?;

    Ok(())
}

fn read_request_body(request: &mut Request<&mut EspHttpConnection>) -> super::Result<String> {
    let len = request
        .content_len()
        .ok_or(super::Error::RequestContentLen)? as usize;

    let mut buffer = vec![0; len];

    request
        .read_exact(&mut buffer)
        .map_err(|e| super::Error::RequestReadBody(e.to_string()))?;

    let body = String::from_utf8_lossy(&buffer);
    Ok(body.to_string())
}

fn send_response(
    request: Request<&mut EspHttpConnection>,
    status_code: u16,
    body: &str,
) -> super::Result<()> {
    let mut response = request
        .into_response(status_code, None, &HEADERS)
        .map_err(super::Error::RequestIntoResponse)?;
    response
        .write_all(body.as_bytes())
        .map_err(super::Error::ResponseWriteAll)?;
    Ok(())
}
