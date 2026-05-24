use std::{
    collections::{BTreeMap, VecDeque},
    time::Duration,
};

use super::DeviceDiag;

const AVG_REQUEST_DURATION_WINDOW: usize = 10;
const LAST_ERRORS_LEN: usize = 10;

#[derive(Clone, Debug)]
pub struct StoreDeviceDiag {
    device_id: String,
    init_completed: bool,
    response_ok_count: usize,
    response_err_count: usize,
    request_durations: VecDeque<Duration>,
    last_errors: BTreeMap<time::OffsetDateTime, String>,
}

impl StoreDeviceDiag {
    pub fn new(device_id: String) -> Self {
        Self {
            device_id,
            init_completed: false,
            response_ok_count: 0,
            response_err_count: 0,
            request_durations: VecDeque::new(),
            last_errors: BTreeMap::new(),
        }
    }

    pub fn msg_device_init_completed(&mut self, duration: Duration) {
        self.init_completed = true;
        self.response_ok_count += 1;
        self.add_request_durations(duration);
    }

    pub fn msg_device_request_ok(&mut self, duration: Duration) {
        self.response_ok_count += 1;
        self.add_request_durations(duration);
    }

    pub fn msg_device_request_err(&mut self, duration: Duration, error: String) {
        self.response_err_count += 1;
        self.add_request_durations(duration);

        while self.last_errors.len() >= LAST_ERRORS_LEN {
            self.last_errors.pop_first();
        }
        self.last_errors
            .insert(time::OffsetDateTime::now_utc(), error);
    }

    fn add_request_durations(&mut self, duration: Duration) {
        while self.request_durations.len() >= AVG_REQUEST_DURATION_WINDOW {
            self.request_durations.pop_front();
        }
        self.request_durations.push_back(duration);
    }
}

impl From<&StoreDeviceDiag> for DeviceDiag {
    fn from(value: &StoreDeviceDiag) -> Self {
        let avg_request_duration = if value.request_durations.is_empty() {
            Duration::default()
        } else {
            value.request_durations.iter().sum::<Duration>() / value.request_durations.len() as u32
        };

        Self {
            init_completed: value.init_completed,
            response_ok_count: value.response_ok_count,
            response_err_count: value.response_err_count,
            avg_request_duration,
            last_errors: value.last_errors.clone(),
        }
    }
}
