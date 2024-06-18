//! Восстановление сохраненного состояния области `static` ПЛК.
//!
//! По сравнению с другими задачами, при выполнении этой задачи выполнение ПЛК приостанавливается,
//! пока не закончится выполнение данной функции

use std::time::Duration;

use serde::Serialize;
use tokio::task::JoinSet;
use tracing::{info, warn};

use crate::{executor::CmpInOut, message::MsgDataBound};

use super::super::{
    config::{ConfigRetention, ConfigRetentionRestoreResult},
    plc::{FunctionBlockBase, IFunctionBlock},
    utils::{join_set_spawn, sleep},
};
pub struct Retention<TMsg, I, Q, S>
where
    TMsg: MsgDataBound,
    I: Clone + Default + Send + Serialize + 'static + Sync,
    Q: Clone + Default + Send + Serialize + 'static + Sync,
    S: Clone + Default + Send + Serialize + 'static + Sync,
    FunctionBlockBase<I, Q, S>: IFunctionBlock<I, Q, S>,
{
    pub cmp_in_out: CmpInOut<TMsg>,
    pub config_retention: Option<ConfigRetention<TMsg, I, Q, S>>,
    pub fb_main: FunctionBlockBase<I, Q, S>,
}

impl<TMsg, I, Q, S> Retention<TMsg, I, Q, S>
where
    TMsg: MsgDataBound + 'static,
    I: Clone + Default + Send + Serialize + 'static + Sync,
    Q: Clone + Default + Send + Serialize + 'static + Sync,
    S: Clone + Default + Send + Serialize + 'static + Sync,
    FunctionBlockBase<I, Q, S>: IFunctionBlock<I, Q, S>,
{
    pub async fn spawn(mut self) -> super::Result<FunctionBlockBase<I, Q, S>> {
        let retention_restore = if let Some(config_retention) = self.config_retention {
            let mut task_set_retention = JoinSet::<ConfigRetentionRestoreResult<S>>::new();

            // Таймаут
            // В tokio есть timeout в модуле time, но использование модуля вызывает панику в WASM.
            let task = task_retention_timeout(config_retention.restore_timeout);
            join_set_spawn(&mut task_set_retention, task);

            task_set_retention.spawn(async move {
                while let Ok(msg) = self.cmp_in_out.recv_input().await {
                    let data = (config_retention.fn_import_static)(&msg);

                    let Ok(data) = data else {
                        return ConfigRetentionRestoreResult::RestoreDeserializationError;
                    };

                    if let Some(data) = data {
                        return ConfigRetentionRestoreResult::RestoreData(data);
                    };
                }
                ConfigRetentionRestoreResult::NoRestoreData
            });

            let mut config_retention = ConfigRetentionRestoreResult::NoRestoreData;
            while let Some(task_result) = task_set_retention.join_next().await {
                config_retention = task_result?;
                task_set_retention.shutdown().await;
            }
            config_retention
        } else {
            ConfigRetentionRestoreResult::NoRestoreData
        };
        match retention_restore {
            ConfigRetentionRestoreResult::NoRestoreData => warn!("Restore retention data: no data"),
            ConfigRetentionRestoreResult::RestoreDeserializationError => {
                warn!("Restore retention data: deserialization error");
            }
            ConfigRetentionRestoreResult::RestoreData(_) => {
                info!("Restore retention data: success")
            }
        }

        let fb_main = match retention_restore {
            ConfigRetentionRestoreResult::NoRestoreData => self.fb_main.clone(),
            ConfigRetentionRestoreResult::RestoreDeserializationError => self.fb_main.clone(),
            ConfigRetentionRestoreResult::RestoreData(stat) => {
                self.fb_main.clone().new_with_restore_stat(stat)
            }
        };

        Ok(fb_main)
    }
}

async fn task_retention_timeout<S>(timeout: Duration) -> ConfigRetentionRestoreResult<S>
where
    S: Clone + Default + Serialize,
{
    sleep(timeout).await;
    ConfigRetentionRestoreResult::NoRestoreData
}
