use crate::components::cmp_plc::plc::FbSystemData;

use super::super::{select_mode, select_sp};

use super::{QHmiPermission, QHmiStatus, QMode, QState, I, Q, S};

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

    // Выбор задания
    stat.mv.call(&mut select_sp::I {
        sp_en_source: input.mv_en_source,
        sp_hmi_en: input.mv_hmi_en,
        sp_plc_en: input.mv_plc_en,
        sp_plc: input.mv_plc,
        hmi_command: input.hmi_command.into(),
    });

    Q {
        hmi_status: QHmiStatus {
            state: QState::default(),
            mode,
            hmi_permission: QHmiPermission {
                man_start: mode == QMode::Manual,
                man_stop: mode == QMode::Manual,

                mode_auto: stat.mode.q.hmi_status.hmi_permission.mode_auto,
                mode_man: stat.mode.q.hmi_status.hmi_permission.mode_man,
                mode_local: stat.mode.q.hmi_status.hmi_permission.mode_local,
                mode_oos: stat.mode.q.hmi_status.hmi_permission.mode_oos,

                mv_hmi_en: stat.mv.q.hmi_status.hmi_permission.sp_hmi_en,
                mv_plc_en: stat.mv.q.hmi_status.hmi_permission.sp_plc_en,
                mv_hmi_plc_en: stat.mv.q.hmi_status.hmi_permission.sp_hmi_plc_en,
                mv_hmi: stat.mv.q.hmi_status.hmi_permission.sp_hmi,
            },

            mv: stat.mv.q.hmi_status.sp,
            mv_plc_act: stat.mv.q.hmi_status.sp_plc_act,
            mv_hmi_act: stat.mv.q.hmi_status.sp_hmi_act,
            rbk: input.rbk,
        },

        mv: stat.mv.q.sp,
        mv_plc_act: stat.mv.q.sp_plc_act,
        mv_hmi_act: stat.mv.q.sp_hmi_act,
    }
}
