use std::{
    collections::HashMap,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

use serde::Deserialize;
use surrealdb_types::SurrealValue;
use tokio::{task::JoinSet, time::sleep};
use tracing::{error, info, warn};

use crate::{
    executor::{ComponentError, MsgBusInput, MsgBusLinker, MsgBusOutput},
    message::MsgDataBound,
};

use super::{Config, DB, Error, tasks};

pub async fn fn_process<TMsg>(
    msgbus_linker: MsgBusLinker<TMsg>,
    config: Config<TMsg>,
) -> Result<(), ComponentError>
where
    TMsg: MsgDataBound + 'static,
{
    info!("Starting Surrealdb");

    let (input, output) = msgbus_linker.input_output();
    msgbus_linker.close();

    let result = task_main(input, output, &config).await;
    match result {
        Ok(_) => error!("SurrealDB stop execution"),
        Err(err) => error!("SurrealDB error: {err}"),
    }

    Ok(())
}

async fn task_main<TMsg>(
    input: MsgBusInput<TMsg>,
    output: MsgBusOutput<TMsg>,
    config: &Config<TMsg>,
) -> super::Result<()>
where
    TMsg: MsgDataBound + 'static,
{
    let connection_established = Arc::new(AtomicBool::new(false));

    let mut task_set: JoinSet<super::Result<()>> = JoinSet::new();

    for request_start_config in &config.request_start {
        let task = tasks::RequestStart {
            msgbus_output: output.clone(),
            start_config: request_start_config.clone(),
        };
        task_set.spawn(task.spawn());
    }

    // Запросы на основе входящих сообщений
    for request_input_config in &config.request_input {
        let task = tasks::RequestInput {
            msgbus_input: input.clone(),
            msgbus_output: output.clone(),
            input_config: request_input_config.clone(),
            connection_established: connection_established.clone(),
        };
        task_set.spawn(task.spawn());
    }

    drop(input);
    drop(output);

    loop {
        let res = connect(config).await;
        match res {
            Ok(_) => {
                connection_established.store(true, Ordering::Release);
                break;
            }
            Err(e) => {
                warn!("SurrealDB connection error: {}", e);
                sleep(Duration::from_millis(1000)).await;
            }
        }
    }

    while let Some(res) = task_set.join_next().await {
        res??
    }
    Ok(())
}

#[derive(Debug, Deserialize, SurrealValue)]
struct InfoRootSystem {
    pub available_parallelism: u32,
    pub cpu_usage: f32,
    pub load_average: Vec<f32>,
    pub memory_allocated: u64,
    pub memory_usage: u64,
    pub physical_cores: u32,
}
#[derive(Debug, Deserialize, SurrealValue)]
struct InfoRoot {
    pub accesses: HashMap<String, String>,
    pub config: HashMap<String, Option<String>>,
    pub defaults: HashMap<String, String>,
    pub namespaces: HashMap<String, String>,
    pub nodes: HashMap<String, String>,
    pub system: InfoRootSystem,
    pub users: HashMap<String, String>,
}

/// Подключение к БД
async fn connect<TMsg>(config: &Config<TMsg>) -> super::Result<()> {
    let address = config.connection.address();
    DB.connect(address).await?;

    // let credentials = Root {
    //     username: config.user.clone(),
    //     password: config.password.clone(),
    // };
    // DB.signin(credentials).await?;

    let mut query = DB.query("INFO FOR ROOT;").await?;
    let info: Option<InfoRoot> = query.take(0)?;
    let info = info.ok_or(Error::QueryInfo)?;
    info!("SurrealDB; info for root: {:?}", info);

    if !info.namespaces.contains_key(&config.namespace) {
        info!("Start executing init script");
        let result = DB.query(&config.init_script).await;
        let mut result = match result {
            Ok(r) => {
                info!("Init script executed");
                r
            }
            Err(err) => {
                error!("Init script failed: {:?}", err);
                return Err(err.into());
            }
        };
        let result_errors = result.take_errors();
        if !result_errors.is_empty() {
            error!("Init script result: {:?}", result_errors);
        }
    }

    DB.use_ns(config.namespace.clone())
        .use_db(config.database.clone())
        .await?;

    Ok(())
}
