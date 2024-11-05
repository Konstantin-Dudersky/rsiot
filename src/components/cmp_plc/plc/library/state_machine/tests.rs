//! Тестирование:
//! ```bash
//! cargo test components::cmp_plc::plc::library::state_machine::tests --features="cmp_plc" --target="x86_64-unknown-linux-gnu"
//! ```

use std::time::Duration;

use serde::Serialize;

#[test]
fn test1() {
    #[derive(Debug, Clone, Copy, Default, PartialEq, Serialize)]
    enum States {
        #[default]
        State1,
        State2,
    }

    let period = Duration::from_millis(100);

    let mut sm = super::FB::new(Duration::from_millis(100));
    let sm_output = sm.call(
        &mut super::I {
            new_state: States::State1,
            cycle_time: Duration::from_millis(100),
        },
        period,
    );

    assert_eq!(sm_output.current_state, States::State1);
    assert_eq!(sm_output.state_time, Duration::from_millis(100));

    // Переходим в состояние State2
    let sm_output = sm.call(
        &mut super::I {
            new_state: States::State2,
            cycle_time: Duration::from_millis(100),
        },
        period,
    );

    assert_eq!(sm_output.current_state, States::State2);
    assert_eq!(sm_output.state_time, Duration::from_millis(0));
    assert!(sm_output.is_first_cycle);

    // Остаемся состояние State2
    let sm_output = sm.call(
        &mut super::I {
            new_state: States::State2,
            cycle_time: Duration::from_millis(100),
        },
        period,
    );

    assert_eq!(sm_output.current_state, States::State2);
    assert_eq!(sm_output.state_time, Duration::from_millis(100));
    assert!(!sm_output.is_first_cycle);

    // Остаемся состояние State2
    let sm_output = sm.call(
        &mut super::I {
            new_state: States::State2,
            cycle_time: Duration::from_millis(100),
        },
        period,
    );

    assert_eq!(sm_output.current_state, States::State2);
    assert_eq!(sm_output.state_time, Duration::from_millis(200));
    assert!(!sm_output.is_first_cycle);
}
