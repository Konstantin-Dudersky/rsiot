use std::{sync::Arc, time::Duration};

use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};
use tokio::{sync::Mutex, task::JoinSet, time::sleep};
use tracing::{error, info};

use crate::{
    executor::{CmpInOut, ComponentError},
    message::MsgDataBound,
};

use super::Config;

type Db = Arc<Mutex<Surreal<Client>>>;

pub async fn fn_process<TMsg>(
    input: CmpInOut<TMsg>,
    config: Config<TMsg>,
) -> Result<(), ComponentError>
where
    TMsg: MsgDataBound + 'static,
{
    info!("Starting Surrealdb");
    loop {
        let result = task_main(input.clone(), &config).await;
        match result {
            Ok(_) => error!("SurrealDB stop execution"),
            Err(err) => error!("SurrealDB error: {err:?}"),
        }
        info!("Restarting...");
        sleep(Duration::from_secs(2)).await;
    }
}

async fn task_main<TMsg>(input: CmpInOut<TMsg>, config: &Config<TMsg>) -> super::Result<()>
where
    TMsg: MsgDataBound + 'static,
{
    let db = connect(config).await?;
    init_script(config, db.clone()).await?;

    let mut tasks: JoinSet<super::Result<()>> = JoinSet::new();

    for item in &config.input_config {
        tasks.spawn(task_periodic_request(
            input.clone(),
            item.clone(),
            db.clone(),
        ));
    }
    while let Some(res) = tasks.join_next().await {
        res??
    }
    Ok(())
}

/// Подключение к БД
async fn connect<TMsg>(config: &Config<TMsg>) -> super::Result<Db> {
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
async fn init_script<TMsg>(config: &Config<TMsg>, db: Db) -> super::Result<()> {
    let db = db.lock().await;
    db.query(config.init_script.clone()).await?;
    Ok(())
}

async fn task_periodic_request<TMsg>(
    mut input: CmpInOut<TMsg>,
    input_config: super::config::InputConfig<TMsg>,
    db: Db,
) -> super::Result<()>
where
    TMsg: MsgDataBound,
{
    while let Ok(msg) = input.recv_input().await {
        let query = (input_config.fn_input)(&msg);
        let query = match query {
            Some(val) => val,
            None => continue,
        };
        {
            let db = db.lock().await;
            let _response = db.query(query).await?;
        }
    }
    Ok(())
}
