use std::sync::{Arc, atomic::AtomicU8};

use tokio::task::JoinSet;

use crate::{
    executor::{CmpInOut, join_set_spawn},
    message::MsgDataBound,
};

use super::{COMPONENT_NAME, Config, tasks};

pub async fn fn_process<TMsg>(
    config: Config<TMsg>,
    msgbus_linker: CmpInOut<TMsg>,
) -> super::Result<()>
where
    TMsg: 'static + MsgDataBound,
{
    let mut task_set: JoinSet<super::Result<()>> = JoinSet::new();

    let partner_live_counter = Arc::new(AtomicU8::new(0));

    // Обновляем счетчик на основе входящих сообщений
    let task = tasks::FindPartnerCounter {
        input: msgbus_linker.input(),
        fn_find_partner_counter: config.fn_find_partner_counter,
        live_counter: partner_live_counter.clone(),
    };
    join_set_spawn(
        &mut task_set,
        format!("{COMPONENT_NAME} | find_partner_counter"),
        task.spawn(),
    );

    // Периодическая проверка счетчика
    let task = tasks::CheckPartnerPeriod {
        output: msgbus_linker.output(),
        fn_check_partner_counter: config.fn_check_partner_counter,
        check_partner_period: config.check_partner_period,
        live_counter: partner_live_counter,
    };
    join_set_spawn(
        &mut task_set,
        format!("{COMPONENT_NAME} | check_partner_period"),
        task.spawn(),
    );

    // Генерирование собственного счетчика
    let task = tasks::GenerateSelfCounter {
        output: msgbus_linker.output(),
        fn_generate_self_counter: config.fn_generate_self_counter,
        generate_self_period: config.generate_self_period,
    };
    join_set_spawn(
        &mut task_set,
        format!("{COMPONENT_NAME} | generate_self_counter"),
        task.spawn(),
    );

    drop(msgbus_linker);

    while let Some(result) = task_set.join_next().await {
        result??;
    }

    Ok(())
}
