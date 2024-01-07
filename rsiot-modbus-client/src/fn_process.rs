use std::{net::SocketAddr, ops::Deref, sync::Arc, time::Instant};

use async_trait::async_trait;
use tokio::{
    sync::Mutex,
    task::JoinSet,
    time::{sleep, Duration},
};
use tokio_modbus::{client::Context, prelude::*};
use tracing::{debug, error, info, trace, warn};

use rsiot_component_core2::{
    Cache, Component, ComponentError, ComponentInput, ComponentOutput, IComponentProcess,
};
use rsiot_messages_core::IMessage;

use crate::{
    config::{self},
    errors::Errors,
    types::Result_,
};

struct Config<TMessage>(crate::config::Config<TMessage>);

impl<TMessage> Deref for Config<TMessage> {
    type Target = crate::config::Config<TMessage>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[async_trait]
impl<TMessage> IComponentProcess<Config<TMessage>, TMessage>
    for Component<Config<TMessage>, TMessage>
where
    TMessage: IMessage,
{
    async fn process(
        &self,
        config: Config<TMessage>,
        input: ComponentInput<TMessage>,
        output: ComponentOutput<TMessage>,
        cache: Cache<TMessage>,
    ) -> Result<(), ComponentError> {
        match &config {
            rsiot_components_config::modbus_client::Config::Tcp(_) => (),
            rsiot_components_config::modbus_client::Config::Rtu => (),
        }

        Ok(())
    }
}

pub async fn fn_process<TMessage>(
    input: ComponentInput<TMessage>,
    output: ComponentOutput<TMessage>,
    config: Config<TMessage>,
    _cache: Cache<TMessage>,
) -> Result<(), ComponentError>
where
    TMessage: IMessage + 'static,
{
    loop {
        info!("Starting modbus client, configuration: {:?}", config);
        let res = task_main::<TMessage>(input.resubscribe(), output.clone(), config.clone()).await;
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
    input: ComponentInput<TMessage>,
    output: ComponentOutput<TMessage>,
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
            input.resubscribe(),
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
    output: ComponentOutput<TMessage>,
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
            periodic_config.period - elapsed
        };
        sleep(sleep_time).await;
    }
}

/// Задача обработки запроса на основе входного потока сообщений
async fn task_input_request<TMessage>(
    mut input: ComponentInput<TMessage>,
    output: ComponentOutput<TMessage>,
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
    output: ComponentOutput<TMessage>,
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
