use std::time::Duration;

use gloo::{
    net::http::{Request, Response},
    timers::future::sleep,
};
use http::StatusCode;
use instant::Instant;
use tokio::task::JoinSet;
use tracing::{error, info};
use url::Url;

use crate::message::{Message, MsgDataBound, ServiceBound};

use crate::executor::CmpInOut;

use super::{config::config, error::Error};

pub async fn fn_process<TMessage, TService>(
    input: CmpInOut<TMessage, TService>,
    config: config::Config<TMessage>,
) -> super::Result<()>
where
    TMessage: MsgDataBound + 'static,
    TService: ServiceBound + 'static,
{
    info!("Starting http-client-wasm, configuration: {:?}", config);

    loop {
        let res = task_main(input.clone(), config.clone()).await;
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

async fn task_main<TMessage, TService>(
    in_out: CmpInOut<TMessage, TService>,
    config: config::Config<TMessage>,
) -> super::Result<()>
where
    TMessage: MsgDataBound + 'static,
    TService: ServiceBound + 'static,
{
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

    let mut task_set = JoinSet::<super::Result<()>>::new();
    // запускаем периодические запросы
    for req in config.requests_periodic {
        let future = task_periodic_request(in_out.clone(), req, url.clone());
        task_set.spawn_local(future);
    }
    for item in config.requests_input {
        let future = task_input_request(in_out.clone(), url.clone(), item);
        task_set.spawn_local(future);
    }
    // TODO - пересмотреть http-client. Может объединить код по-максимуму? NewType на основе
    // JoineSet - с возможностью выбора spawn или spawn_local
    while let Some(res) = task_set.join_next().await {
        res??
    }
    Ok(())
}

/// Задача обработки запросов на основе входящего потока сообщений
async fn task_input_request<TMessage, TService>(
    mut in_out: CmpInOut<TMessage, TService>,
    url: Url,
    config: config::RequestInput<TMessage>,
) -> super::Result<()>
where
    TMessage: MsgDataBound,
    TService: ServiceBound,
{
    while let Ok(msg) = in_out.recv_input().await {
        let http_param = (config.fn_input)(&msg);
        let http_param = match http_param {
            Some(val) => val,
            None => continue,
        };
        let msgs =
            process_request_and_response(&url, &http_param, config.on_success, config.on_failure)
                .await?;
        for msg in msgs {
            in_out.send_output(msg).await.map_err(Error::CmpOutput)?;
        }
    }
    Ok(())
}

/// Задача обработки периодического запроса
async fn task_periodic_request<TMessage, TService>(
    output: CmpInOut<TMessage, TService>,
    config: config::RequestPeriodic<TMessage>,
    url: Url,
) -> super::Result<()>
where
    TMessage: MsgDataBound,
    TService: ServiceBound,
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
            output.send_output(msg).await.map_err(Error::CmpOutput)?;
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
async fn process_request_and_response<TMessage>(
    url: &Url,
    request_param: &config::HttpParam,
    on_success: config::CbkOnSuccess<TMessage>,
    on_failure: config::CbkOnFailure<TMessage>,
) -> super::Result<Vec<Message<TMessage>>>
where
    TMessage: MsgDataBound,
{
    let response = send_request(url.clone(), request_param).await;
    let response = match response {
        Ok(val) => val,
        Err(err) => match err {
            Error::GlooNet(source) => {
                error!("{:?}", source);
                let msgs = (on_failure)();
                return Ok(msgs);
            }
            _ => return Err(err),
        },
    };
    let status = response.status();
    let text = response.text().await?;
    if status != StatusCode::OK {
        let msgs = (on_failure)();
        error!(
            "Error on request.\nRequest params: {:?}\nResponse text: {:?}\nStatus: {:?}",
            request_param, text, status
        );
        return Ok(msgs);
    }
    let msgs = (on_success)(&text).map_err(Error::OnSuccess)?;
    Ok(msgs)
}

/// Выполнение HTTP запроса
async fn send_request(url: Url, req: &config::HttpParam) -> super::Result<Response> {
    let endpoint = match req {
        config::HttpParam::Get { endpoint } => endpoint,
        config::HttpParam::Put { endpoint, body: _ } => endpoint,
        config::HttpParam::Post { endpoint, body: _ } => endpoint,
    };
    let url = url
        .join(endpoint)
        .map_err(|err| Error::Configuration(err.to_string()))?;
    let response = match req {
        config::HttpParam::Get { endpoint: _ } => Request::get(url.as_ref()).send().await?,
        config::HttpParam::Put { endpoint: _, body } => {
            Request::put(url.as_ref()).body(body)?.send().await?
        }
        config::HttpParam::Post { endpoint: _, body } => {
            Request::post(url.as_ref()).body(body)?.send().await?
        }
    };
    Ok(response)
}
