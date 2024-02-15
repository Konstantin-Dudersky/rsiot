use std::time::{Duration, Instant};

use gloo::{
    net::http::{Request, Response},
    timers::future::sleep,
};
use http::StatusCode;
use tokio::task::JoinSet;
use tracing::{error, info};
use url::Url;

use rsiot_component_core::{CmpInput, CmpOutput};
use rsiot_messages_core::IMessage;

use crate::{config::config, error::Error};

pub async fn fn_process<TMessage>(
    input: CmpInput<TMessage>,
    output: CmpOutput<TMessage>,
    config: config::Config<TMessage>,
) -> crate::Result<()>
where
    TMessage: IMessage + 'static,
{
    info!("Starting http-client-wasm, configuration: {:?}", config);

    loop {
        let res = task_main::<TMessage>(input.clone(), output.clone(), config.clone()).await;
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

async fn task_main<TMessage>(
    _input: CmpInput<TMessage>,
    output: CmpOutput<TMessage>,
    config: config::Config<TMessage>,
) -> crate::Result<()>
where
    TMessage: IMessage + 'static,
{
    let mut task_set = JoinSet::<crate::Result<()>>::new();
    // запускаем периодические запросы
    for req in config.requests_periodic {
        let future = task_periodic_request::<TMessage>(
            output.clone(),
            req,
            config.connection_config.base_url.clone(),
        );
        task_set.spawn_local(future);
    }
    // TODO - запросы на основе входящих сообщений
    //
    // TODO - пересмотреть http-client. Может объединить код по-максимуму? NewType на основе
    // JoineSet - с возможностью выбора spawn или spawn_local
    while let Some(res) = task_set.join_next().await {
        res??
    }
    Ok(())
}

/// Задача обработки периодического запроса
async fn task_periodic_request<TMessage>(
    output: CmpOutput<TMessage>,
    config: config::RequestPeriodic<TMessage>,
    url: Url,
) -> crate::Result<()>
where
    TMessage: IMessage,
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
            output.send(msg).await.map_err(Error::CmpOutput)?;
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
) -> crate::Result<Vec<TMessage>>
where
    TMessage: IMessage,
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
            "Error on request.\nRequest params: {:?}\nResponse text: {:?}",
            request_param, text
        );
        return Ok(msgs);
    }
    let msgs = (on_success)(&text).map_err(Error::OnSuccess)?;
    Ok(msgs)
}

/// Выполнение HTTP запроса
async fn send_request(url: Url, req: &config::HttpParam) -> crate::Result<Response> {
    let endpoint = match req {
        config::HttpParam::Get(val) => val,
        config::HttpParam::Put(_) => todo!(),
        config::HttpParam::Post(_) => todo!(),
    };
    let url = url
        .join(endpoint)
        .map_err(|err| Error::Configuration(err.to_string()))?;
    let response = match req {
        config::HttpParam::Get(_) => Request::get(url.as_ref()).send().await?,
        config::HttpParam::Put(_) => todo!(),
        config::HttpParam::Post(_) => todo!(),
    };
    Ok(response)
}
