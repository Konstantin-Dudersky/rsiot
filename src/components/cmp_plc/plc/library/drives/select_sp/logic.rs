use super::{IHmiCommand, QHmiPermission, QHmiStatus, I, Q, S};

pub fn logic(input: &I, stat: &mut S) -> Q {
    stat.sp_plc_act = match input.sp_en_source {
        // из hmi
        false => match input.hmi_command {
            IHmiCommand::no_command => stat.sp_plc_act,
            IHmiCommand::sp_hmi_en => false,
            IHmiCommand::sp_plc_en => true,
            IHmiCommand::sp_hmi(_) => stat.sp_plc_act,
        },

        // из plc
        true => {
            if input.sp_hmi_en {
                false
            } else if input.sp_plc_en {
                true
            } else {
                stat.sp_plc_act
            }
        }
    };

    if let IHmiCommand::sp_hmi(sp) = input.hmi_command {
        stat.sp_hmi = sp
    };

    let sp = match stat.sp_plc_act {
        true => {
            stat.sp_hmi = input.sp_plc;
            input.sp_plc
        }
        false => stat.sp_hmi,
    };

    Q {
        hmi_status: QHmiStatus {
            hmi_permission: QHmiPermission {
                sp_hmi_en: !input.sp_en_source && stat.sp_plc_act,
                sp_plc_en: !input.sp_en_source && !stat.sp_plc_act,
                sp_hmi_plc_en: !input.sp_en_source,
                sp_hmi: !stat.sp_plc_act,
            },
            sp_plc_act: stat.sp_plc_act,
            sp_hmi_act: !stat.sp_plc_act,
            sp,
        },
        sp_plc_act: stat.sp_plc_act,
        sp_hmi_act: !stat.sp_plc_act,
        sp,
    }
}
