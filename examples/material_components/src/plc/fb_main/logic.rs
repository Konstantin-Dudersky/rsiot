use super::{I, Q, S};

use rsiot::components::cmp_plc::plc::library::drives;

pub fn logic(input: &I, stat: &mut S) -> Q {
    stat.m1.call(drives::motor::I {
        mode_source: false,
        mode_auto: false,
        mode_man: false,
        mode_local: false,
        mode_oos: false,
        hmi_command: input.m1_command,
        auto_start: false,
        auto_stop: false,
    });

    Q {
        m1_status: stat.m1.output.hmi_status,
    }
}
