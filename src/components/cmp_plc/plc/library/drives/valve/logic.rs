use super::super::select_mode;

use super::{QHmiPermission, QHmiStatus, QMode, QState, I, Q, S};

pub fn logic(input: &I, stat: &mut S) -> Q {
    // Выбор режима
    stat.mode.call(select_mode::I {
        mode_source: input.mode_source,
        mode_auto: input.mode_auto,
        mode_man: input.mode_man,
        mode_local: false,
        mode_oos: false,
        hmi_command: input.hmi_command.into(),
    });
    let mode = stat.mode.output.mode;

    Q {
        hmi_status: QHmiStatus {
            state: QState::default(),
            mode,
            hmi_permission: QHmiPermission {
                man_start: mode == QMode::Manual,
                man_stop: mode == QMode::Manual,
                mode_auto: stat.mode.output.hmi_status.hmi_permission.mode_auto,
                mode_man: stat.mode.output.hmi_status.hmi_permission.mode_man,
                mode_local: stat.mode.output.hmi_status.hmi_permission.mode_local,
                mode_oos: stat.mode.output.hmi_status.hmi_permission.mode_oos,
            },
        },
    }
}
