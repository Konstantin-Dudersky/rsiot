use super::{IHmiCommand, QHmiPermission, QHmiStatus, QMode, QState, I, Q, S};

use super::super::mode_select;

pub fn logic(input: &I, stat: &mut S) -> Q {
    // Выбор режима
    stat.mode.call(mode_select::I {
        mode_plc_hmi: input.mode_plc_hmi,
        auto_mode_plc: input.auto_mode_plc,
        man_mode_plc: input.man_mode_plc,
        local_mode_plc: false,
        oos_mode_plc: false,
        hmi_command: match input.hmi_command {
            IHmiCommand::NoCommand => mode_select::IHmiCommand::NoCommand,
            IHmiCommand::ManMode => mode_select::IHmiCommand::ManMode,
            IHmiCommand::AutoMode => mode_select::IHmiCommand::AutoMode,
            IHmiCommand::LocalMode => mode_select::IHmiCommand::LocalMode,
            IHmiCommand::ManStart | IHmiCommand::ManStop => mode_select::IHmiCommand::NoCommand,
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

                auto_mode: stat.mode.output.hmi_status.hmi_permission.auto_mode,
                man_mode: stat.mode.output.hmi_status.hmi_permission.auto_mode,
                local_mode: stat.mode.output.hmi_status.hmi_permission.auto_mode,
                oos_mode: stat.mode.output.hmi_status.hmi_permission.auto_mode,
            },
            mode,
            state: stat.state,
        },
        start: stat.state == QState::Start,
    }
}
