use super::{event, EventSeverity};

/// Определить макс. уровень из текущих аварий
pub fn define_max_severity(events: &[event::QHmiStatus]) -> EventSeverity {
    let mut max_severity = EventSeverity::default();
    for event in events {
        if event.state == event::State::NoAct_Ack {
            continue;
        }
        if event.event_severity > max_severity {
            max_severity = event.event_severity.clone();
        }
    }
    max_severity
}
