use std::sync::{Arc, atomic::AtomicU8};

use tokio::task::JoinSet;

use crate::{
    executor::{MsgBusInput, MsgBusOutput, join_set_spawn},
    message::MsgDataBound,
};

use super::{Config, tasks};

pub async fn fn_process<TMsg>(
    config: Config<TMsg>,
    input: MsgBusInput<TMsg>,
    output: MsgBusOutput<TMsg>,
) -> super::Result<()>
where
    TMsg: 'static + MsgDataBound,
{
    let mut task_set: JoinSet<super::Result<()>> = JoinSet::new();

    let partner_live_counter = Arc::new(AtomicU8::new(0));

    // Обновляем счетчик на основе входящих сообщений
    let task = tasks::FindPartnerCounter {
        input,
        fn_find_partner_counter: config.fn_find_partner_counter,
        live_counter: partner_live_counter.clone(),
    };
    join_set_spawn(&mut task_set, "cmp_livecounter", task.spawn());

    // Периодическая проверка счетчика
    let task = tasks::CheckPartnerPeriod {
        output: output.clone(),
        fn_check_partner_counter: config.fn_check_partner_counter,
        check_partner_period: config.check_partner_period,
        live_counter: partner_live_counter,
    };
    join_set_spawn(&mut task_set, "cmp_livecounter", task.spawn());

    // Генерирование собственного счетчика
    let task = tasks::GenerateSelfCounter {
        output,
        fn_generate_self_counter: config.fn_generate_self_counter,
        generate_self_period: config.generate_self_period,
    };
    join_set_spawn(&mut task_set, "cmp_livecounter", task.spawn());

    while let Some(result) = task_set.join_next().await {
        result??;
    }

    Ok(())
}
