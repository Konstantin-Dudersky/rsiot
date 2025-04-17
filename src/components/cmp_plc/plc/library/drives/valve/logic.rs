use crate::components::cmp_plc::plc::FbSystemData;

use super::super::select_mode;

use super::{IHmiCommand, QHmiPermission, QHmiStatus, QMode, QState, I, Q, S};

pub fn logic(input: &I, stat: &mut S, _system_data: &FbSystemData) -> Q {
    // Выбор режима
    stat.mode.call(&mut select_mode::I {
        mode_source: input.mode_source,
        mode_auto: input.mode_auto,
        mode_man: input.mode_man,
        mode_local: false,
        mode_oos: false,
        hmi_command: input.hmi_command.into(),
    });
    let mode = stat.mode.q.mode;

    // Команда открыть / закрыть
    stat.control = match mode {
        QMode::Auto => {
            if input.auto_close {
                false
            } else {
                input.auto_open
            }
        }
        QMode::Local => false,
        QMode::Manual => match input.hmi_command {
            IHmiCommand::man_open => true,
            IHmiCommand::man_close => false,
            _ => stat.control,
        },
        QMode::Oos => false,
    };

    let state = {
        if stat.control {
            QState::Opened
        } else {
            QState::Closed
        }
    };

    Q {
        control: stat.control,
        hmi_status: QHmiStatus {
            state,
            mode,
            hmi_permission: QHmiPermission {
                man_start: mode == QMode::Manual,
                man_stop: mode == QMode::Manual,
                mode_auto: stat.mode.q.hmi_status.hmi_permission.mode_auto,
                mode_man: stat.mode.q.hmi_status.hmi_permission.mode_man,
                mode_local: stat.mode.q.hmi_status.hmi_permission.mode_local,
                mode_oos: stat.mode.q.hmi_status.hmi_permission.mode_oos,
            },
            control: stat.control,
        },
    }
}
