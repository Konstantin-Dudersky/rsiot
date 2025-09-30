use std::time::Duration;

use embedded_svc::http::{Method, client::Client as HttpClient};
use esp_idf_svc::http::client::EspHttpConnection;
use tokio::{
    task::JoinSet,
    time::{Instant, sleep},
};
use tracing::{error, info, warn};
use url::Url;

use crate::{
    components::shared_tasks::cmp_http_client::HttpClientGeneral,
    executor::{MsgBusLinker, join_set_spawn},
    message::{Message, MsgDataBound},
};

use super::{Error, config};

pub async fn fn_process<TMsg>(
    config: config::Config<TMsg>,
    msg_bus: MsgBusLinker<TMsg>,
) -> super::Result<()>
where
    TMsg: MsgDataBound + 'static,
{
    // Необходимо подождать, пока поднимется Wi-Fi
    sleep(Duration::from_secs(2)).await;

    info!("Starting http-client, configuration: {:?}", config);

    let mut task_set = JoinSet::new();

    let http_client_general = HttpClientGeneral {
        msg_bus,
        buffer_size: 100,
        task_set: &mut task_set,
        requests_input: config.requests_input,
        requests_periodic: config.requests_periodic,
    };

    let (ch_rx_requests, ch_tx_reponse) = http_client_general.spawn();

    let task = tasks::HttpClient {
        input: ch_rx_requests,
        output: ch_tx_reponse,
        base_url: config.base_url,
        timeout: config.timeout,
    };
    join_set_spawn(&mut task_set, task.spawn());

    while let Some(res) = task_set.join_next().await {
        res??;
    }
    Ok(())
}

/// Основная задача
async fn task_main<TMsg>(
    in_out: MsgBusLinker<TMsg>,
    config: config::Config<TMsg>,
) -> super::Result<()>
where
    TMsg: MsgDataBound + 'static,
{
    let mut set = JoinSet::<super::Result<()>>::new();

    // Парсим url
    let url = Url::parse(&config.base_url);
    let url = match url {
        Ok(val) => val,
        Err(err) => {
            let err = err.to_string();
            let err = format!("Cannot parse url: {}", err);
            return Err(Error::Configuration(err));
        }
    };

    // запускаем периодические запросы
    for req in config.requests_periodic {
        let future = task_periodic_request(in_out.clone(), req, url.clone());
        set.spawn_local(future);
    }
    // Запускаем задачи запросов на основе входного потока сообщений
    // for item in config.requests_input {
    //     let future = task_input_request(
    //         in_out.clone(),
    //         config.connection_config.base_url.clone(),
    //         item,
    //     );
    //     set.spawn(future);
    // }
    while let Some(res) = set.join_next().await {
        res??
    }
    Ok(())
}

/// Задача обработки периодического запроса
async fn task_periodic_request<TMsg>(
    in_out: MsgBusLinker<TMsg>,
    config: config::RequestPeriodic<TMsg>,
    url: Url,
) -> super::Result<()>
where
    TMsg: MsgDataBound,
{
    loop {
        let begin = Instant::now();

        let msgs = process_request_and_response(
            &url,
            &config.http_param,
            config.on_success,
            config.on_failure,
        )
        .await?;
        for msg in msgs {
            in_out.send_output(msg).await?;
        }
        let elapsed = begin.elapsed();
        let sleep_time = if config.period <= elapsed {
            Duration::from_millis(10)
        } else {
            config.period - elapsed
        };

        sleep(sleep_time).await;
    }
}

/// Выполнение запроса и вызов коллбеков при ответе
async fn process_request_and_response<TMsg>(
    url: &Url,
    request_param: &config::HttpParam,
    on_success: config::CbkOnSuccess<TMsg>,
    on_failure: config::CbkOnFailure<TMsg>,
) -> super::Result<Vec<Message<TMsg>>> {
    info!("Call http client");
    let response = send_request(url.clone(), request_param).await;
    // let response = match response {
    //     Ok(val) => val,
    //     Err(err) => match err {
    //         Error::Reqwest(source) => {
    //             error!("{:?}", source);
    //             let msgs = (on_failure)();
    //             return Ok(msgs);
    //         }
    //         _ => return Err(err),
    //     },
    // };
    // let status = response.status();
    // let text = response.text().await?;
    // if status != StatusCode::OK {
    //     let msgs = (on_failure)();
    //     error!(
    //         "Error on request.\nRequest params: {:?}\nResponse text: {:?}",
    //         request_param, text
    //     );
    //     return Ok(msgs);
    // }
    // let msgs = (on_success)(&text)?;

    let msgs = vec![];
    Ok(msgs)
}

/// Выполнение HTTP запроса
async fn send_request(url: Url, req: &config::HttpParam) -> super::Result<()> {
    let endpoint = match req {
        config::HttpParam::Get { endpoint } => endpoint,
        config::HttpParam::Put { endpoint, body: _ } => endpoint,
        config::HttpParam::Post { endpoint, body: _ } => endpoint,
    };
    let url = url.join(endpoint).map_err(|err| {
        let err = err.to_string();
        Error::Configuration(err)
    })?;
    let url = url.to_string();
    info!("Url: {}", url);

    let mut client = HttpClient::wrap(EspHttpConnection::new(&Default::default()).unwrap());

    let headers = [("accept", "text/plain")];
    let response = match req {
        config::HttpParam::Get { endpoint: _ } => {
            let request = client.request(Method::Get, url.as_ref(), &headers);
            let request = match request {
                Ok(val) => val,
                Err(err) => {
                    let err = err.to_string();
                    warn!("{}", err);
                    return Ok(());
                }
            };
            request.submit().unwrap()
            // client.get(&url).unwrap().submit().unwrap()
        }
        // config::HttpParam::Put { endpoint: _, body } => {
        //     client.put(url).body(body.to_string()).send().await?
        // }
        // config::HttpParam::Post { endpoint: _, body } => {
        //     client.post(url).body(body.to_string()).send().await?
        // }
        _ => todo!(),
    };
    let status = response.status();
    info!("<- {}", status);
    Ok(())
}
