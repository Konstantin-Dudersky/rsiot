use super::{I, Q, S};

use rsiot::components::cmp_plc::plc::library::drives;

pub fn logic(input: &I, stat: &mut S) -> Q {
    stat.m1.call(drives::motor::I {
        mode_source: false,
        mode_auto: false,
        mode_man: false,
        mode_local: false,
        mode_oos: false,
        hmi_command: input.motor_hmi_command,
        auto_start: false,
        auto_stop: false,
    });

    stat.v1.call(drives::valve_analog::I {
        mode_source: false,
        mode_auto: false,
        mode_man: false,
        mode_local: false,
        mode_oos: false,
        mv_en_source: false,
        mv_hmi_en: false,
        mv_plc_en: false,
        mv_plc: 0.0,
        rbk: stat.v1.output.mv,
        hmi_command: input.valve_analog_hmi_command,
    });

    stat.valve.call(drives::valve::I {
        mode_source: false,
        mode_auto: false,
        mode_man: false,
        mode_local: false,
        mode_oos: false,
        auto_open: false,
        auto_close: false,
        hmi_command: input.valve_hmi_command,
    });

    Q {
        motor_hmi_status: stat.m1.output.hmi_status,
        valve_analog_hmi_status: stat.v1.output.hmi_status,
        valve_hmi_status: stat.valve.output.hmi_status,
    }
}
