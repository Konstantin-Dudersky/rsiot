use std::time::Duration;

use reqwest::{Client, Response, StatusCode};
use tokio::{
    spawn,
    sync::{broadcast, mpsc},
    task::JoinSet,
    time::{sleep, Instant},
};
use tracing::{error, info};
use url::Url;

use rsiot_component_core::{IComponent, Input, Output};
use rsiot_extra_components::{cmp_mpsc_to_mpsc, cmpbase_mpsc_to_broadcast};
use rsiot_messages_core::IMessage;

use crate::{config::config, error::Error, types::Result_};

pub async fn process<TMessage>(
    input: Input<TMessage>,
    output: Output<TMessage>,
    config: config::Config<TMessage>,
) where
    TMessage: IMessage + 'static,
{
    info!("Starting http-client, configuration: {:?}", config);

    // Канал для распространения входного потока сообщений по порождаемым задачам
    let (from_input_tx, _from_inpit_rx) = broadcast::channel::<TMessage>(100);
    let _task_from_input = spawn(cmpbase_mpsc_to_broadcast::new(input, from_input_tx.clone()));

    // Канал для сбора выходных потоков из порожденных задач в один
    let (to_output_tx, to_output_rx) = mpsc::channel::<TMessage>(100);
    let _task_to_output = cmp_mpsc_to_mpsc::create().set_and_spawn(Some(to_output_rx), output);

    loop {
        let res =
            task_main::<TMessage>(from_input_tx.clone(), to_output_tx.clone(), config.clone())
                .await;
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
async fn task_main<TMessage>(
    input: broadcast::Sender<TMessage>,
    output: mpsc::Sender<TMessage>,
    config: config::Config<TMessage>,
) -> Result_<(), TMessage>
where
    TMessage: IMessage + 'static,
{
    let mut set = JoinSet::<Result_<(), TMessage>>::new();
    // запускаем периодические запросы
    for req in config.requests_periodic {
        let future = task_periodic_request::<TMessage>(
            output.clone(),
            req,
            config.connection_config.base_url.clone(),
        );
        set.spawn(future);
    }
    // Запускаем задачи запросов на основе входного потока сообщений
    for item in config.requests_input {
        let future = task_input_request(
            input.subscribe(),
            output.clone(),
            config.connection_config.base_url.clone(),
            item,
        );
        set.spawn(future);
    }
    while let Some(res) = set.join_next().await {
        res??
    }
    Ok(())
}

/// Задача обработки периодического запроса
async fn task_periodic_request<TMessage>(
    output: mpsc::Sender<TMessage>,
    config: config::RequestPeriodic<TMessage>,
    url: Url,
) -> Result_<(), TMessage>
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
            output.send(msg).await?;
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

/// Задача обработки запросов на основе входящего потока сообщений
async fn task_input_request<TMessage>(
    mut input: broadcast::Receiver<TMessage>,
    output: mpsc::Sender<TMessage>,
    url: Url,
    config: config::RequestInput<TMessage>,
) -> Result_<(), TMessage>
where
    TMessage: IMessage,
{
    while let Ok(msg) = input.recv().await {
        let http_param = (config.fn_input)(&msg);
        let http_param = match http_param {
            Some(val) => val,
            None => continue,
        };
        let msgs =
            process_request_and_response(&url, &http_param, config.on_success, config.on_failure)
                .await?;
        for msg in msgs {
            output.send(msg).await?;
        }
    }
    Ok(())
}

/// Выполнение запроса и вызов коллбеков при ответе
async fn process_request_and_response<TMessage>(
    url: &Url,
    request_param: &config::HttpParam,
    on_success: config::CbkOnSuccess<TMessage>,
    on_failure: config::CbkOnFailure<TMessage>,
) -> Result_<Vec<TMessage>, TMessage>
where
    TMessage: IMessage,
{
    let response = send_request(url.clone(), request_param).await;
    let response = match response {
        Ok(val) => val,
        Err(err) => match err {
            Error::Reqwest(err) => {
                error!("{:?}", err);
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
    let msgs = (on_success)(&text);
    Ok(msgs)
}

/// Выполнение HTTP запроса
async fn send_request<TMessage>(url: Url, req: &config::HttpParam) -> Result_<Response, TMessage> {
    let endpoint = match req {
        config::HttpParam::Get(val) => val,
        config::HttpParam::Put(_) => todo!(),
        config::HttpParam::Post(_) => todo!(),
    };
    let url = url.join(endpoint).map_err(|err| {
        let err = err.to_string();
        Error::Configuration(err)
    })?;
    let client = Client::new();
    let response = match req {
        config::HttpParam::Get(_) => client.get(url).send().await?,
        config::HttpParam::Put(_) => todo!(),
        config::HttpParam::Post(_) => todo!(),
    };
    Ok(response)
}
