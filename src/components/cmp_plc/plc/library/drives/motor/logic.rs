use crate::components::cmp_plc::plc::FbSystemData;

use super::{IHmiCommand, QHmiPermission, QHmiStatus, QMode, QState, I, Q, S};

use super::super::select_mode;

pub fn logic(input: &I, stat: &mut S, system_data: &FbSystemData) -> Q {
    // Выбор режима
    stat.mode.call(
        &mut select_mode::I {
            mode_source: input.mode_source,
            mode_auto: input.mode_auto,
            mode_man: input.mode_man,
            mode_local: false,
            mode_oos: false,
            hmi_command: input.hmi_command.into(),
        },
        system_data.period,
    );
    let mode = stat.mode.output.mode;

    // Команда на запуск
    stat.state = match mode {
        QMode::Auto => {
            if input.auto_stop {
                QState::Stop
            } else if input.auto_start {
                QState::Start
            } else {
                stat.state
            }
        }
        QMode::Local => QState::Stop,
        QMode::Manual => match input.hmi_command {
            IHmiCommand::man_start => QState::Start,
            IHmiCommand::man_stop => QState::Stop,
            _ => stat.state,
        },
        QMode::Oos => QState::Stop,
    };

    // Блокировка работы
    stat.state = match input.intlock {
        true => stat.state,
        false => QState::Stop,
    };

    Q {
        hmi_status: QHmiStatus {
            hmi_permission: QHmiPermission {
                man_start: mode == QMode::Manual,
                man_stop: mode == QMode::Manual,

                mode_auto: stat.mode.output.hmi_status.hmi_permission.mode_auto,
                mode_man: stat.mode.output.hmi_status.hmi_permission.mode_man,
                mode_local: stat.mode.output.hmi_status.hmi_permission.mode_local,
                mode_oos: stat.mode.output.hmi_status.hmi_permission.mode_oos,
            },
            mode,
            state: stat.state,
            start: stat.state == QState::Start,
        },
        start: stat.state == QState::Start,
    }
}
