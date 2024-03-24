use std::{net::SocketAddr, sync::Arc, time::Instant};

use tokio::{
    sync::Mutex,
    task::JoinSet,
    time::{sleep, Duration},
};
use tokio_modbus::{client::Context, prelude::*};
use tracing::{debug, error, info, trace, warn};

use crate::{
    executor::{CmpInOut, ComponentError},
    message::MsgDataBound,
};

use super::{
    config::{self, Config},
    error::Error,
};

pub async fn fn_process<TMessage>(
    in_out: CmpInOut<TMessage>,
    config: Config<TMessage>,
) -> Result<(), ComponentError>
where
    TMessage: MsgDataBound + 'static,
{
    if !config.enabled {
        loop {
            warn!("Service disabled");
            sleep(Duration::from_secs(u64::max_value())).await
        }
    }

    loop {
        info!("Starting modbus client, configuration: {:?}", config);
        let res = task_main::<TMessage>(in_out.clone(), config.clone()).await;
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
    in_out: CmpInOut<TMessage>,
    config: Config<TMessage>,
) -> super::Result<()>
where
    TMessage: MsgDataBound + 'static,
{
    let ctx = match config.connection_config {
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

    let mut set: JoinSet<super::Result<()>> = JoinSet::new();

    // Запускаем задачи периодических запросов
    for item in config.periodic_config {
        set.spawn(task_periodic_request::<TMessage>(
            in_out.clone(),
            ctx.clone(),
            item,
        ));
    }
    // Запускаем задачи запросов на основе входного потока сообщений
    for item in config.input_config {
        set.spawn(task_input_request(in_out.clone(), ctx.clone(), item));
    }
    while let Some(res) = set.join_next().await {
        res??
    }
    Ok(())
}

/// Задача обработки периодического запроса
async fn task_periodic_request<TMessage>(
    in_out: CmpInOut<TMessage>,
    ctx: Arc<Mutex<Context>>,
    periodic_config: config::PeriodicConfig<TMessage>,
) -> super::Result<()>
where
    TMessage: MsgDataBound,
{
    loop {
        let begin = Instant::now();
        let response = modbus_request(ctx.clone(), &periodic_config.request).await;
        modbus_response(
            in_out.clone(),
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
    mut in_out: CmpInOut<TMessage>,
    ctx: Arc<Mutex<Context>>,
    input_config: config::InputConfig<TMessage>,
) -> super::Result<()>
where
    TMessage: MsgDataBound,
{
    while let Ok(msg) = in_out.recv_input().await {
        let request = (input_config.fn_input)(&msg);
        let request = match request {
            Some(val) => val,
            None => continue,
        };
        let response = modbus_request(ctx.clone(), &request).await;
        modbus_response(
            in_out.clone(),
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
) -> super::Result<config::Response> {
    let mut lock = ctx.lock().await;
    match request {
        config::Request::ReadCoils(_, _) => todo!(),
        config::Request::ReadHoldingRegisters(start_address, count) => {
            let response = lock.read_holding_registers(*start_address, *count).await?;
            Ok(config::Response::WordVector(response))
        }
        config::Request::WriteSingleRegister(start_address, value) => {
            lock.write_single_register(*start_address, *value).await?;
            Ok(config::Response::Unit)
        }
    }
}

/// Обратываем ответ modbus
async fn modbus_response<TMessage>(
    output: CmpInOut<TMessage>,
    request: &config::Request,
    response: &super::Result<config::Response>,
    fn_on_success: config::FnOnSuccess<TMessage>,
    fn_on_failure: config::FnOnFailure<TMessage>,
) -> super::Result<()>
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
        output.send_output(msg).await.map_err(Error::CmpOutput)?;
    }
    Ok(())
}
