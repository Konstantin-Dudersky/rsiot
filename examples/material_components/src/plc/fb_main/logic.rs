use super::{I, Q, S};

use rsiot::components::cmp_plc::plc::{library::drives, FbSystemData};

pub fn logic(input: &I, stat: &mut S, system_data: &FbSystemData) -> Q {
    stat.motor.call(
        &mut drives::motor::I {
            mode_source: false,
            mode_auto: false,
            mode_man: false,
            mode_local: false,
            mode_oos: false,
            hmi_command: input.motor_hmi_command,
            auto_start: false,
            auto_stop: false,
            intlock: true,
        },
        system_data.period,
    );

    stat.valve_analog.call(
        &mut drives::valve_analog::I {
            mode_source: false,
            mode_auto: false,
            mode_man: false,
            mode_local: false,
            mode_oos: false,
            mv_en_source: false,
            mv_hmi_en: false,
            mv_plc_en: false,
            mv_plc: 0.0,
            rbk: stat.valve_analog.output.mv,
            hmi_command: input.valve_analog_hmi_command,
        },
        system_data.period,
    );

    stat.valve.call(
        &mut drives::valve::I {
            mode_source: false,
            mode_auto: false,
            mode_man: false,
            mode_local: false,
            mode_oos: false,
            auto_open: false,
            auto_close: false,
            hmi_command: input.valve_hmi_command,
        },
        system_data.period,
    );

    Q {
        motor_hmi_status: stat.motor.output.hmi_status,
        valve_analog_hmi_status: stat.valve_analog.output.hmi_status,
        valve_hmi_status: stat.valve.output.hmi_status,
    }
}
