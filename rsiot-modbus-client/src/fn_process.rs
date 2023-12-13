use std::{net::SocketAddr, sync::Arc, time::Instant};

use tokio::{
    spawn,
    sync::{broadcast, mpsc, Mutex},
    task::JoinSet,
    time::{sleep, Duration},
};
use tokio_modbus::{client::Context, prelude::*};
use tracing::{debug, error, info, trace, warn};

use rsiot_component_core::{IComponent, StreamInput, StreamOutput};
use rsiot_components_config::modbus_client as config;
use rsiot_extra_components::{cmp_mpsc_to_mpsc, cmpbase_mpsc_to_broadcast};
use rsiot_messages_core::IMessage;

use crate::{config::Config, errors::Errors, types::Result_};

pub async fn fn_process<TMessage>(
    input: StreamInput<TMessage>,
    output: StreamOutput<TMessage>,
    config: Config<TMessage>,
) where
    TMessage: IMessage + 'static,
{
    // Канал для распространения входного потока сообщений по порождаемым задачам
    let (from_input_tx, _from_inpit_rx) = broadcast::channel::<TMessage>(100);
    let _task_from_input = spawn(cmpbase_mpsc_to_broadcast::new(input, from_input_tx.clone()));

    // Канал для сбора выходных потоков из порожденных задач в один
    let (to_output_tx, to_output_rx) = mpsc::channel::<TMessage>(100);
    let _task_to_output = cmp_mpsc_to_mpsc::create().set_and_spawn(Some(to_output_rx), output);

    loop {
        info!("Starting modbus client, configuration: {:?}", config);
        let res =
            task_main::<TMessage>(from_input_tx.clone(), to_output_tx.clone(), config.clone())
                .await;
        match res {
            Ok(_) => (),
            Err(err) => {
                error!("Error in Modbus client: {:?}", err);
            }
        }
        info!("Restarting...");
        sleep(Duration::from_secs(2)).await;
    }
}

async fn task_main<TMessage>(
    input: broadcast::Sender<TMessage>,
    output: mpsc::Sender<TMessage>,
    client_config: Config<TMessage>,
) -> Result<(), Errors>
where
    TMessage: IMessage + 'static,
{
    let (ctx, periodic_config, input_config) = match client_config {
        Config::Tcp(config) => {
            let socket_addr = SocketAddr::new(config.host, config.port);
            debug!("Try to establish connection to socket: {:?}", socket_addr);
            let slave = Slave(config.unit_id);
            let ctx = tcp::connect_slave(socket_addr, slave).await?;
            debug!("Connection established: {:?}", ctx);
            (ctx, config.periodic_config, config.input_config)
        }
        Config::Rtu => {
            todo!();
        }
    };
    let ctx = Arc::new(Mutex::new(ctx));

    let mut set = JoinSet::<Result_<()>>::new();

    // Запускаем задачи периодических запросов
    for item in periodic_config {
        set.spawn(task_periodic_request::<TMessage>(
            output.clone(),
            ctx.clone(),
            item,
        ));
    }
    // Запускаем задачи запросов на основе входного потока сообщений
    for item in input_config {
        set.spawn(task_input_request(
            input.subscribe(),
            output.clone(),
            ctx.clone(),
            item,
        ));
    }
    while let Some(res) = set.join_next().await {
        res??
    }
    Ok(())
}

/// Задача обработки периодического запроса
async fn task_periodic_request<TMessage>(
    output: mpsc::Sender<TMessage>,
    ctx: Arc<Mutex<Context>>,
    periodic_config: config::PeriodicConfig<TMessage>,
) -> Result_<()>
where
    TMessage: IMessage,
{
    loop {
        let begin = Instant::now();
        let response = modbus_request(ctx.clone(), &periodic_config.request).await;
        modbus_response(
            output.clone(),
            &periodic_config.request,
            &response,
            periodic_config.fn_on_success,
            periodic_config.fn_on_failure,
        )
        .await?;
        let elapsed = begin.elapsed();
        let sleep_time = if periodic_config.period <= elapsed {
            Duration::from_millis(10)
        } else {
            periodic_config.period - begin.elapsed()
        };
        sleep(sleep_time).await;
    }
}

/// Задача обработки запроса на основе входного потока сообщений
async fn task_input_request<TMessage>(
    mut input: broadcast::Receiver<TMessage>,
    output: mpsc::Sender<TMessage>,
    ctx: Arc<Mutex<Context>>,
    input_config: config::InputConfig<TMessage>,
) -> Result_<()>
where
    TMessage: IMessage,
{
    while let Ok(msg) = input.recv().await {
        let request = (input_config.fn_input)(&msg);
        let request = match request {
            Some(val) => val,
            None => continue,
        };
        let response = modbus_request(ctx.clone(), &request).await;
        modbus_response(
            output.clone(),
            &request,
            &response,
            input_config.fn_on_success,
            input_config.fn_on_failure,
        )
        .await?;
    }
    Ok(())
}

/// Выполняем запрос modbus
async fn modbus_request(
    ctx: Arc<Mutex<Context>>,
    request: &config::Request,
) -> Result_<config::Response> {
    let mut lock = ctx.lock().await;
    match request {
        config::Request::ReadCoils(_, _) => todo!(),
        config::Request::ReadHoldingRegisters(start_address, count) => {
            let response = lock.read_holding_registers(*start_address, *count).await?;
            Ok(config::Response::U16(response))
        }
        config::Request::WriteSingleRegister(start_address, value) => {
            lock.write_single_register(*start_address, *value).await?;
            Ok(config::Response::Unit)
        }
    }
}

/// Обратываем ответ modbus
async fn modbus_response<TMessage>(
    output: mpsc::Sender<TMessage>,
    request: &config::Request,
    response: &Result<config::Response, Errors>,
    fn_on_success: config::FnOnSuccess<TMessage>,
    fn_on_failure: config::FnOnFailure<TMessage>,
) -> Result_<()> {
    trace!("Modbus response: {:?}", response);
    let msgs = match response {
        Ok(val) => (fn_on_success)(val),
        Err(err) => {
            let err = format!(
                "Modbus request error. Request: {:?}. Error: {:?}",
                request, err
            );
            warn!(err);
            (fn_on_failure)()
        }
    };
    for msg in msgs {
        output.send(msg).await?;
    }
    Ok(())
}
