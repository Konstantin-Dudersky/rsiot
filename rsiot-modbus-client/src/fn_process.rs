use std::{net::SocketAddr, sync::Arc, time::Instant};

use tokio::{
    sync::Mutex,
    task::JoinSet,
    time::{sleep, Duration},
};
use tokio_modbus::{client::Context, prelude::*};
use tracing::{debug, error, info, trace};

use rsiot_component_core::{Cache, CmpInput, CmpOutput, ComponentError};
use rsiot_messages_core::MsgDataBound;

use crate::{
    config::{self, Config},
    error::Error,
};

pub async fn fn_process<TMessage>(
    input: CmpInput<TMessage>,
    output: CmpOutput<TMessage>,
    config: Config<TMessage>,
    _cache: Cache<TMessage>,
) -> Result<(), ComponentError>
where
    TMessage: MsgDataBound + 'static,
{
    if config.enabled {
        loop {
            sleep(Duration::from_secs(100)).await
        }
    }

    loop {
        info!("Starting modbus client, configuration: {:?}", config);
        let res = task_main::<TMessage>(input.clone(), output.clone(), config.clone()).await;
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
    input: CmpInput<TMessage>,
    output: CmpOutput<TMessage>,
    config: Config<TMessage>,
) -> crate::Result<()>
where
    TMessage: MsgDataBound + 'static,
{
    let ctx = match config.client_type {
        config::ClientType::Tcp(tcp_config) => {
            let socket_addr = SocketAddr::new(tcp_config.host, tcp_config.port);
            debug!("Try to establish connection to socket: {:?}", socket_addr);
            let slave = Slave(config.unit_id);
            let ctx = tcp::connect_slave(socket_addr, slave).await?;
            debug!("Connection established: {:?}", ctx);
            ctx
        }
        config::ClientType::Rtu => {
            unimplemented!()
        }
    };
    let ctx = Arc::new(Mutex::new(ctx));

    let mut set: JoinSet<crate::Result<()>> = JoinSet::new();

    // Запускаем задачи периодических запросов
    for item in config.periodic_config {
        set.spawn(task_periodic_request::<TMessage>(
            output.clone(),
            ctx.clone(),
            item,
        ));
    }
    // Запускаем задачи запросов на основе входного потока сообщений
    for item in config.input_config {
        set.spawn(task_input_request(
            input.clone(),
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
    output: CmpOutput<TMessage>,
    ctx: Arc<Mutex<Context>>,
    periodic_config: config::PeriodicConfig<TMessage>,
) -> crate::Result<()>
where
    TMessage: MsgDataBound,
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
            periodic_config.period - elapsed
        };
        sleep(sleep_time).await;
    }
}

/// Задача обработки запроса на основе входного потока сообщений
async fn task_input_request<TMessage>(
    mut input: CmpInput<TMessage>,
    output: CmpOutput<TMessage>,
    ctx: Arc<Mutex<Context>>,
    input_config: config::InputConfig<TMessage>,
) -> crate::Result<()>
where
    TMessage: MsgDataBound,
{
    while let Ok(msg) = input.recv().await {
        let msg = match msg {
            Some(val) => val,
            None => continue,
        };
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
) -> crate::Result<config::Response> {
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
    output: CmpOutput<TMessage>,
    request: &config::Request,
    response: &crate::Result<config::Response>,
    fn_on_success: config::FnOnSuccess<TMessage>,
    fn_on_failure: config::FnOnFailure<TMessage>,
) -> crate::Result<()>
where
    TMessage: MsgDataBound,
{
    trace!("Modbus response: {:?}", response);
    let msgs = match response {
        Ok(val) => (fn_on_success)(val),
        Err(err) => {
            (fn_on_failure)();
            let err = Error::Request {
                request: request.clone(),
                error: err.to_string(),
            };
            return Err(err);
        }
    };
    for msg in msgs {
        output.send(msg).await.map_err(Error::CmpOutput)?;
    }
    Ok(())
}
