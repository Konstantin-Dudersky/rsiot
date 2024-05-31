use super::{IHmiCommand, QHmiPermission, QHmiStatus, SMode, I, Q, S};

pub fn logic(input: &I, stat: &mut S) -> Q {
    // Выбор режима
    match input.mode_plc_hmi {
        // Задание с hmi
        true => match input.hmi_command {
            IHmiCommand::AutoMode => stat.mode = SMode::Auto,
            IHmiCommand::ManMode => stat.mode = SMode::Manual,
            _ => (),
        },

        // Задание с plc
        false => {
            if input.auto_mode_plc {
                stat.mode = SMode::Auto;
            } else if input.man_mode_plc {
                stat.mode = SMode::Manual;
            }
        }
    }

    // Команда на запуск
    stat.start = match stat.mode {
        SMode::Auto => {
            if input.auto_start {
                true
            } else if input.auto_stop {
                false
            } else {
                false
            }
        }
        SMode::Local => false,
        SMode::Manual => match input.hmi_command {
            IHmiCommand::ManStart => true,
            IHmiCommand::ManStop => false,
            _ => stat.start,
        },
        SMode::Oos => false,
    };

    Q {
        status: QHmiStatus {
            man_act: stat.mode == SMode::Manual,
            aut_act: stat.mode == SMode::Auto,
            hmi_permission: QHmiPermission {
                man_start: stat.mode == SMode::Manual,
                man_stop: stat.mode == SMode::Manual,
            },
        },
        start: stat.start,
    }
}
