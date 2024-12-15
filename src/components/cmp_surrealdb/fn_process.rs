use std::{sync::Arc, time::Duration};

use surrealdb::{engine::remote::ws::Ws, opt::auth::Root, Surreal};
use tokio::{sync::Mutex, task::JoinSet, time::sleep};
use tracing::{debug, error, info};

use crate::{
    executor::{CmpInOut, ComponentError},
    message::{MsgDataBound, ServiceBound},
};

use super::{tasks, Config, DbClient};

pub async fn fn_process<TMsg, TService>(
    input: CmpInOut<TMsg, TService>,
    config: Config<TMsg>,
) -> Result<(), ComponentError>
where
    TMsg: MsgDataBound + 'static,
    TService: ServiceBound + 'static,
{
    info!("Starting Surrealdb");
    loop {
        let result = task_main(input.clone(), &config).await;
        match result {
            Ok(_) => error!("SurrealDB stop execution"),
            Err(err) => error!("SurrealDB error: {err}"),
        }
        info!("Restarting...");
        sleep(Duration::from_secs(2)).await;
    }
}

async fn task_main<TMsg, TService>(
    input: CmpInOut<TMsg, TService>,
    config: &Config<TMsg>,
) -> super::Result<()>
where
    TMsg: MsgDataBound + 'static,
    TService: ServiceBound + 'static,
{
    let db = connect(config).await?;
    init_script(config, db.clone()).await?;

    let mut task_set: JoinSet<super::Result<()>> = JoinSet::new();

    for request_start_config in &config.request_start {
        let task = tasks::RequestStart {
            in_out: input.clone(),
            start_config: request_start_config.clone(),
            db_client: db.clone(),
        };
        task_set.spawn(task.spawn());
    }

    // Запросы на основе входящих сообщений
    for request_input_config in &config.request_input {
        let task = tasks::RequestInput {
            in_out: input.clone(),
            input_config: request_input_config.clone(),
            db_client: db.clone(),
        };
        task_set.spawn(task.spawn());
    }

    while let Some(res) = task_set.join_next().await {
        res??
    }
    Ok(())
}

/// Подключение к БД
async fn connect<TMsg>(config: &Config<TMsg>) -> super::Result<DbClient> {
    let url = format!("{}:{}", config.host, config.port);
    let db = Surreal::new::<Ws>(url).await?;

    let credentials = Root {
        username: &config.user,
        password: &config.password,
    };
    db.signin(credentials).await?;

    db.use_ns(config.namespace.clone())
        .use_db(config.database.clone())
        .await?;

    Ok(Arc::new(Mutex::new(db)))
}

/// Выполнение первоначального скрипта
async fn init_script<TMsg>(config: &Config<TMsg>, db: DbClient) -> super::Result<()> {
    debug!("Execute init script");
    let db = db.lock().await;
    db.query(config.init_script.clone()).await?;
    debug!("Init script completed");
    Ok(())
}
