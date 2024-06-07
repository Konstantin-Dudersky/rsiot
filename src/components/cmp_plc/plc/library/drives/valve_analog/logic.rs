use super::super::{mode_select, sp_plc_hmi};

use super::{IHmiCommand, QHmiPermission, QHmiStatus, QMode, QState, I, Q, S};

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
            IHmiCommand::OosMode => mode_select::IHmiCommand::OosMode,

            _ => mode_select::IHmiCommand::NoCommand,
        },
    });
    let mode = stat.mode.output.mode;

    // Выбор задания
    stat.mv.call(sp_plc_hmi::I {
        sp_en_select: input.mv_en_select,
        sp_hmi_en: input.mv_hmi_en,
        sp_plc_en: input.mv_plc_en,
        sp_plc: input.mv_plc,
        hmi_command: match input.hmi_command {
            IHmiCommand::NoCommand => sp_plc_hmi::IHmiCommand::no_command,
            IHmiCommand::mv_hmi_en => todo!(),
            IHmiCommand::mv_plc_en => todo!(),
            IHmiCommand::mv_hmi(_) => todo!(),
            _ => sp_plc_hmi::IHmiCommand::no_command,
        },
    });

    Q {
        hmi_status: QHmiStatus {
            state: QState::default(),
            mode,
            hmi_permission: QHmiPermission {
                man_start: mode == QMode::Manual,
                man_stop: mode == QMode::Manual,

                auto_mode: stat.mode.output.hmi_status.hmi_permission.auto_mode,
                man_mode: stat.mode.output.hmi_status.hmi_permission.man_mode,
                local_mode: stat.mode.output.hmi_status.hmi_permission.local_mode,
                oos_mode: stat.mode.output.hmi_status.hmi_permission.oos_mode,

                mv_hmi_en: stat.mv.output.hmi_status.hmi_permission.sp_hmi_en,
                mv_plc_en: stat.mv.output.hmi_status.hmi_permission.sp_plc_en,
                mv_hmi_plc_en: stat.mv.output.hmi_status.hmi_permission.sp_hmi_plc_en,
                mv_hmi: stat.mv.output.hmi_status.hmi_permission.sp_hmi,
            },

            mv: stat.mv.output.hmi_status.sp,
            mv_plc_act: stat.mv.output.hmi_status.sp_plc_act,
            mv_hmi_act: stat.mv.output.hmi_status.sp_hmi_act,
        },

        mv: stat.mv.output.sp,
        mv_plc_act: stat.mv.output.sp_plc_act,
        mv_hmi_act: stat.mv.output.sp_hmi_act,
    }
}
