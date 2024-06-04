use super::{IHmiCommand, QHmiPermission, QHmiStatus, QMode, QState, I, Q, S};

pub fn logic(input: &I, stat: &mut S) -> Q {
    // Выбор режима
    match input.mode_plc_hmi {
        // Задание с hmi
        true => match input.hmi_command {
            IHmiCommand::AutoMode => stat.mode = QMode::Auto,
            IHmiCommand::ManMode => stat.mode = QMode::Manual,
            _ => (),
        },

        // Задание с plc
        false => {
            if input.auto_mode_plc {
                stat.mode = QMode::Auto;
            } else if input.man_mode_plc {
                stat.mode = QMode::Manual;
            }
        }
    }

    // Команда на запуск
    stat.state = match stat.mode {
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
                man_start: stat.mode == QMode::Manual,
                man_stop: stat.mode == QMode::Manual,
                auto_mode: stat.mode != QMode::Auto && !input.mode_plc_hmi,
                man_mode: stat.mode != QMode::Manual && !input.mode_plc_hmi,
                local_mode: stat.mode != QMode::Local && !input.mode_plc_hmi,
                oos_mode: stat.mode != QMode::Oos && !input.mode_plc_hmi,
            },
            mode: stat.mode,
            state: stat.state,
        },
        start: stat.state == QState::Start,
    }
}
