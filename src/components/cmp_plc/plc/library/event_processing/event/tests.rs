//! Тестирование:
//! ```bash
//! cargo test components::cmp_plc::plc::library::event_processing::event::tests --features="cmp_plc" --target="x86_64-unknown-linux-gnu";
//! ```

use std::time::Duration;

use super::*;
use uuid::uuid;

#[test]
fn act_ack_noact() {
    let id = uuid!("16f7eed7-fb01-4208-9ebb-84cd1993d464");

    let mut ev = FB::new(Duration::from_millis(100));
    assert_eq!(ev.output.state, State::NoAct_Ack);

    // Событие не наступило
    ev.call(&mut I {
        id,
        signal: false,
        hmi_command: IHmiCommand::NoCommand,
        ..Default::default()
    });
    assert_eq!(ev.output.state, State::NoAct_Ack);

    // Событие наступило
    ev.call(&mut I {
        id,
        signal: true,
        hmi_command: IHmiCommand::NoCommand,
        ..Default::default()
    });
    assert_eq!(ev.output.state, State::Act_NoAck);

    // Квитируем другое событие
    ev.call(&mut I {
        id,
        signal: true,
        hmi_command: IHmiCommand::Ack(uuid!("47d7fb4b-797c-4ae6-b067-120aa418f115")),
        ..Default::default()
    });
    assert_eq!(ev.output.state, State::Act_NoAck);

    // Квитируем данное событие
    ev.call(&mut I {
        id,
        signal: true,
        hmi_command: IHmiCommand::Ack(id),
        ..Default::default()
    });
    assert_eq!(ev.output.state, State::Act_Ack);

    // Событие ушло
    ev.call(&mut I {
        id,
        signal: false,
        hmi_command: IHmiCommand::Ack(id),
        ..Default::default()
    });
    assert_eq!(ev.output.state, State::NoAct_Ack);
}

#[test]
fn act_noact_ack() {
    let id = uuid!("16f7eed7-fb01-4208-9ebb-84cd1993d464");

    let mut ev = FB::new(Duration::from_millis(100));
    assert_eq!(ev.output.state, State::NoAct_Ack);

    // Событие не наступило
    ev.call(&mut I {
        id,
        signal: false,
        hmi_command: IHmiCommand::NoCommand,
        ..Default::default()
    });
    assert_eq!(ev.output.state, State::NoAct_Ack);

    // Событие наступило
    ev.call(&mut I {
        id,
        signal: true,
        hmi_command: IHmiCommand::NoCommand,
        ..Default::default()
    });
    assert_eq!(ev.output.state, State::Act_NoAck);

    // Событие ушло
    ev.call(&mut I {
        id,
        signal: false,
        hmi_command: IHmiCommand::NoCommand,
        ..Default::default()
    });
    assert_eq!(ev.output.state, State::NoAct_NoAck);

    // Квитируем данное событие
    ev.call(&mut I {
        id,
        signal: false,
        hmi_command: IHmiCommand::Ack(id),
        ..Default::default()
    });
    assert_eq!(ev.output.state, State::NoAct_Ack);
}
