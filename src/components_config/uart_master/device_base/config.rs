use std::time::Duration;

pub struct ConfigPeriodicRequest<TRequest, TBuffer> {
    pub period: Duration,
    pub fn_request: fn(&TBuffer) -> TRequest,
}
