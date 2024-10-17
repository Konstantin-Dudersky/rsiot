use std::time::Duration;

use super::{I, Q, S};

pub fn logic<TState>(input: &I<TState>, stat: &mut S<TState>) -> Q<TState>
where
    TState: Copy + PartialEq,
{
    let is_first_cycle;
    if stat.current_state == input.new_state {
        stat.state_time += input.cycle_time;
        is_first_cycle = false;
    } else {
        stat.state_time = Duration::default();
        stat.current_state = input.new_state;
        is_first_cycle = true;
    };

    Q {
        current_state: stat.current_state,
        state_time: stat.state_time,
        is_first_cycle,
    }
}
