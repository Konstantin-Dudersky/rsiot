use std::time::Duration;

use embedded_svc::http::{client::Client as HttpClient, Method};
use esp_idf_svc::http::client::EspHttpConnection;
use tokio::{
    task::JoinSet,
    time::{sleep, Instant},
};
use tracing::{error, info, warn};
use url::Url;

use crate::{
    executor::CmpInOut,
    message::{Message, MsgDataBound},
};

use super::{config, Error};

pub async fn fn_process<TMsg>(
    config: config::Config<TMsg>,
    in_out: CmpInOut<TMsg>,
) -> super::Result<()>
where
    TMsg: MsgDataBound + 'static,
{
    // Необходимо подождать, пока поднимется Wi-Fi
    sleep(Duration::from_secs(2)).await;

    info!("Starting http-client, configuration: {:?}", config);

    loop {
        let res = task_main(in_out.clone(), config.clone()).await;
        match res {
            Ok(_) => (),
            Err(err) => {
                error!("Error in http-client: {:?}", err);
            }
        }
        info!("Restarting...");
        sleep(Duration::from_secs(2)).await;
    }
}

/// Основная задача
async fn task_main<TMsg>(in_out: CmpInOut<TMsg>, config: config::Config<TMsg>) -> super::Result<()>
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
    in_out: CmpInOut<TMsg>,
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
