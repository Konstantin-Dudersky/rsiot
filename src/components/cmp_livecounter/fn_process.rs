use std::sync::{atomic::AtomicU8, Arc};

use futures::TryFutureExt;
use tokio::{sync::mpsc, task::JoinSet};

use crate::{
    components::shared_tasks,
    executor::{join_set_spawn, CmpInOut},
    message::{MsgDataBound, ServiceBound},
};

use super::{tasks, Config};

const BUFFER_SIZE: usize = 100;

pub async fn fn_process<TMsg, TService>(
    config: Config<TMsg>,
    msg_bus: CmpInOut<TMsg, TService>,
) -> super::Result<()>
where
    TMsg: 'static + MsgDataBound,
    TService: 'static + ServiceBound,
{
    let mut task_set: JoinSet<super::Result<()>> = JoinSet::new();

    let partner_live_counter = Arc::new(AtomicU8::new(0));

    let (ch_tx_msgbus_to_input, ch_rx_msgbus_to_input) = mpsc::channel(BUFFER_SIZE);
    let (ch_tx_to_msgbus, ch_rx_to_msgbus) = mpsc::channel(BUFFER_SIZE);

    // Передаем входящие сообщения в канал mpsc
    let task = shared_tasks::msgbus_to_mpsc::MsgBusToMpsc {
        msg_bus: msg_bus.clone(),
        output: ch_tx_msgbus_to_input,
    };
    join_set_spawn(
        &mut task_set,
        task.spawn().map_err(super::Error::TaskMsgBusToMpsc),
    );

    // Обновляем счетчик на основе входящих сообщений
    let task = tasks::FindPartnerCounter {
        input: ch_rx_msgbus_to_input,
        fn_find_partner_counter: config.fn_find_partner_counter,
        live_counter: partner_live_counter.clone(),
    };
    join_set_spawn(&mut task_set, task.spawn());

    // Периодическая проверка счетчика
    let task = tasks::CheckPartnerPeriod {
        output: ch_tx_to_msgbus.clone(),
        fn_check_partner_counter: config.fn_check_partner_counter,
        check_partner_period: config.check_partner_period,
        live_counter: partner_live_counter,
    };
    join_set_spawn(&mut task_set, task.spawn());

    // Передача сообщений на выход компонента
    let task = shared_tasks::mpsc_to_msgbus::MpscToMsgBus {
        input: ch_rx_to_msgbus,
        msg_bus: msg_bus.clone(),
    };
    join_set_spawn(
        &mut task_set,
        task.spawn().map_err(super::Error::TaskMpscToMsgBus),
    );

    // Генерирование собственного счетчика
    let task = tasks::GenerateSelfCounter {
        output: ch_tx_to_msgbus,
        fn_generate_self_counter: config.fn_generate_self_counter,
        generate_self_period: config.generate_self_period,
    };
    join_set_spawn(&mut task_set, task.spawn());

    while let Some(result) = task_set.join_next().await {
        result??;
    }

    Ok(())
}
