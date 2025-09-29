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
    components_config::http_server::{GetEndpointsCollection, PutEndpointsCollection},
    executor::{CmpInOut, MsgBusOutput, join_set_spawn},
    message::MsgDataBound,
};

use super::{Config, config::handler_info, tasks};

/// Заголовки для разрешения CORS
const HEADERS: [(&str, &str); 4] = [
    ("Access-Control-Allow-Origin", "*"),
    ("Access-Control-Max-Age", "600"),
    ("Access-Control-Allow-Methods", "PUT,POST,GET,OPTIONS"),
    ("Access-Control-Allow-Headers", "*"),
];

pub async fn fn_process<TMsg>(
    msgbus_linker: CmpInOut<TMsg>,
    config: Config<TMsg>,
) -> super::Result<()>
where
    TMsg: MsgDataBound + 'static,
{
    let get_endpoints = GetEndpointsCollection::new(&config.get_endpoints);
    let get_endpoints_paths = get_endpoints.all_paths();
    let get_endpoints = Arc::new(Mutex::new(get_endpoints));

    let put_endpoints = PutEndpointsCollection::new(&config.put_endpoints);
    let put_endpoints_paths = put_endpoints.all_paths();
    let put_endpoints = Arc::new(Mutex::new(put_endpoints));

    // Необходимо подождать, пока поднимется Wi-Fi
    while let Ok(msg) = msgbus_linker.input().recv().await {
        let Some(msg) = msg.get_custom_data() else {
            continue;
        };

        let start = (config.fn_start)(&msg);

        if let Some(start) = start
            && start
        {
            break;
        } else {
            continue;
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
        input: msgbus_linker.input(),
        get_endpoints: get_endpoints.clone(),
    };
    join_set_spawn(&mut task_set, "cmp_esp_http_server", task.spawn());

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
        let msgbus_output = msgbus_linker.output();
        server
            .fn_handler(&path, Method::Put, move |request| {
                route_put(request, put_endpoints.clone(), msgbus_output.clone())
            })
            .map_err(super::Error::RegisterHandler)?;
    }

    // Запросы POST
    for path in put_endpoints_paths {
        let put_endpoints = put_endpoints.clone();
        let msgbus_output = msgbus_linker.output();
        server
            .fn_handler(&path, Method::Post, move |request| {
                route_put(request, put_endpoints.clone(), msgbus_output.clone())
            })
            .map_err(super::Error::RegisterHandler)?;
    }

    drop(msgbus_linker);

    // Ждем выполнения всех задач ------------------------------------------------------------------
    while let Some(res) = task_set.join_next().await {
        res??
    }
    Ok(())
}

fn route_root<TMsg>(
    request: Request<&mut EspHttpConnection>,
    get_endpoints: Arc<Mutex<GetEndpointsCollection<TMsg>>>,
    put_endpoints: Arc<Mutex<PutEndpointsCollection<TMsg>>>,
) -> super::Result<()>
where
    TMsg: MsgDataBound,
{
    let body = handler_info(
        &get_endpoints.blocking_lock(),
        &put_endpoints.blocking_lock(),
    );

    send_response(request, 200, body.as_bytes())?;
    Ok(())
}

fn route_get<TMsg>(
    request: Request<&mut EspHttpConnection>,
    get_endpoints: Arc<Mutex<GetEndpointsCollection<TMsg>>>,
) -> super::Result<()>
where
    TMsg: MsgDataBound,
{
    let path = request.uri();
    trace!("Get request, path: {}", path);

    let response_bytes = {
        let get_endpoints = get_endpoints.blocking_lock();
        get_endpoints.handler(path, super::Error::UnknownPath, super::Error::Serde)?
    };

    send_response(request, 200, &response_bytes)?;
    Ok(())
}

fn route_put<TMsg>(
    mut request: Request<&mut EspHttpConnection>,
    put_endpoints: Arc<Mutex<PutEndpointsCollection<TMsg>>>,
    output: MsgBusOutput<TMsg>,
) -> super::Result<()>
where
    TMsg: MsgDataBound,
{
    let path = &request.uri().to_string();
    trace!("Put request, path: {}", path);

    let body = read_request_body(&mut request)?;

    let msg = {
        let put_endpoints = put_endpoints.blocking_lock();
        put_endpoints.handler(path, &body, super::Error::UnknownPath, super::Error::Serde)
    };

    let msg = match msg {
        Ok(val) => val,
        Err(err) => {
            send_response(request, 400, err.to_string().as_bytes())?;
            return Err(err);
        }
    };

    let Some(msg) = msg else { return Ok(()) };

    output.send_blocking(msg)?;

    Ok(())
}

fn read_request_body(request: &mut Request<&mut EspHttpConnection>) -> super::Result<Vec<u8>> {
    let len = request
        .content_len()
        .ok_or(super::Error::RequestContentLen)? as usize;

    let mut buffer = vec![0; len];

    request
        .read_exact(&mut buffer)
        .map_err(|e| super::Error::RequestReadBody(e.to_string()))?;

    Ok(buffer)
}

fn send_response(
    request: Request<&mut EspHttpConnection>,
    status_code: u16,
    body: &[u8],
) -> super::Result<()> {
    let mut response = request
        .into_response(status_code, None, &HEADERS)
        .map_err(super::Error::RequestIntoResponse)?;
    response
        .write_all(body)
        .map_err(super::Error::ResponseWriteAll)?;
    Ok(())
}
