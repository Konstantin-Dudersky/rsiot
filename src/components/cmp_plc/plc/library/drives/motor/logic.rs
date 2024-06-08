use super::{IHmiCommand, QHmiPermission, QHmiStatus, QMode, QState, I, Q, S};

use super::super::select_mode;

pub fn logic(input: &I, stat: &mut S) -> Q {
    // Выбор режима
    stat.mode.call(select_mode::I {
        mode_source: input.mode_source,
        mode_auto: input.mode_auto,
        mode_man: input.mode_man,
        mode_local: false,
        mode_oos: false,
        hmi_command: match input.hmi_command {
            IHmiCommand::NoCommand => select_mode::IHmiCommand::no_command,
            IHmiCommand::ManMode => select_mode::IHmiCommand::mode_man,
            IHmiCommand::AutoMode => select_mode::IHmiCommand::mode_auto,
            IHmiCommand::LocalMode => select_mode::IHmiCommand::mode_local,
            IHmiCommand::ManStart | IHmiCommand::ManStop => select_mode::IHmiCommand::no_command,
        },
    });
    let mode = stat.mode.output.mode;

    // Команда на запуск
    stat.state = match stat.mode.output.mode {
        QMode::Auto => {
            if input.auto_start {
                QState::Start
            } else if input.auto_stop {
                QState::Stop
            } else {
                QState::Stop
            }
        }
        QMode::Local => QState::Stop,
        QMode::Manual => match input.hmi_command {
            IHmiCommand::ManStart => QState::Start,
            IHmiCommand::ManStop => QState::Stop,
            _ => stat.state,
        },
        QMode::Oos => QState::Stop,
    };

    Q {
        hmi_status: QHmiStatus {
            hmi_permission: QHmiPermission {
                man_start: mode == QMode::Manual,
                man_stop: mode == QMode::Manual,

                auto_mode: stat.mode.output.hmi_status.hmi_permission.mode_auto,
                man_mode: stat.mode.output.hmi_status.hmi_permission.mode_man,
                local_mode: stat.mode.output.hmi_status.hmi_permission.mode_local,
                oos_mode: stat.mode.output.hmi_status.hmi_permission.mode_oos,
            },
            mode,
            state: stat.state,
        },
        start: stat.state == QState::Start,
    }
}
