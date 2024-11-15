use std::time::Duration;

pub struct ConfigPeriodicRequest<TRequest> {
    pub period: Duration,
    pub request: TRequest,
}

pub struct ConfigInputRequest {}
