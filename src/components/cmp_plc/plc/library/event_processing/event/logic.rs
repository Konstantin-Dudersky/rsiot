use super::{IHmiCommand, QHmiStatus, State, I, Q, S};

pub fn logic(input: &I, stat: &mut S) -> Q {
    match stat.state {
        State::NoAct_Ack => {
            if input.signal {
                stat.state = State::Act_NoAck
            }
        }
        State::Act_NoAck => {
            match input.hmi_command {
                IHmiCommand::NoCommand => (),
                IHmiCommand::Ack(id) => {
                    if id == input.id {
                        stat.state = State::Act_Ack;
                    }
                }
                IHmiCommand::AckAll => stat.state = State::Act_Ack,
            };
            if !input.signal {
                stat.state = State::NoAct_NoAck;
            };
        }
        State::Act_Ack => {
            if !input.signal {
                stat.state = State::NoAct_Ack
            }
        }
        State::NoAct_NoAck => {
            match input.hmi_command {
                IHmiCommand::NoCommand => (),
                IHmiCommand::Ack(id) => {
                    if id == input.id {
                        stat.state = State::NoAct_Ack;
                    }
                }
                IHmiCommand::AckAll => stat.state = State::Act_Ack,
            };
            if input.signal {
                stat.state = State::Act_NoAck
            }
        }
    }

    Q {
        hmi_status: QHmiStatus {
            state: stat.state,
            text: input.text.clone(),
            event_severity: input.event_severity.clone(),
            id: input.id,
        },
        state: stat.state,
    }
}
