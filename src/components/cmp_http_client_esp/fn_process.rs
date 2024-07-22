use std::time::Duration;

use embedded_svc::{
    http::{client::Client as HttpClient, Method},
    io::Write,
    utils::io,
};
use tokio::{
    task::JoinSet,
    time::{sleep, Instant},
};
use tracing::{error, info};
use url::Url;

use crate::{executor::CmpInOut, message::MsgDataBound};

use super::{config, Error};

pub async fn fn_process<TMsg>(
    config: config::Config<TMsg>,
    in_out: CmpInOut<TMsg>,
) -> super::Result<()>
where
    TMsg: MsgDataBound + 'static,
{
    info!("Starting http-client, configuration: {:?}", config);

    loop {
        let res = task_main::<TMsg>(in_out.clone(), config.clone()).await;
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
    // запускаем периодические запросы
    for req in config.requests_periodic {
        let future = task_periodic_request::<TMsg>(
            in_out.clone(),
            req,
            config.connection_config.base_url.clone(),
        );
        set.spawn(future);
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
    let response = send_request(url.clone(), request_param).await;
    let response = match response {
        Ok(val) => val,
        Err(err) => match err {
            Error::Reqwest(source) => {
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
            "Error on request.\nRequest params: {:?}\nResponse text: {:?}",
            request_param, text
        );
        return Ok(msgs);
    }
    let msgs = (on_success)(&text)?;
    Ok(msgs)
}

/// Выполнение HTTP запроса
async fn send_request<TMessage>(url: Url, req: &config::HttpParam) -> super::Result<Response> {
    let endpoint = match req {
        config::HttpParam::Get { endpoint } => endpoint,
        config::HttpParam::Put { endpoint, body: _ } => endpoint,
        config::HttpParam::Post { endpoint, body: _ } => endpoint,
    };
    let url = url.join(endpoint).map_err(|err| {
        let err = err.to_string();
        Error::Configuration(err)
    })?;
    let client = Client::new();
    let response = match req {
        config::HttpParam::Get { endpoint: _ } => client.get(url).send().await?,
        config::HttpParam::Put { endpoint: _, body } => {
            client.put(url).body(body.to_string()).send().await?
        }
        config::HttpParam::Post { endpoint: _, body } => {
            client.post(url).body(body.to_string()).send().await?
        }
    };
    Ok(response)
}
