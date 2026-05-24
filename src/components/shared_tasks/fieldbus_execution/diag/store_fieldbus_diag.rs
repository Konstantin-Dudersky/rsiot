use std::{
    collections::{HashMap, hash_map::Entry},
    time::Duration,
};

use crate::executor::Instant;

use super::{DeviceDiag, FieldbusDiag, StoreDeviceDiag};

#[derive(Clone, Debug)]
pub struct StoreFieldbusDiag {
    start_time: Instant,
    response_ok_count: usize,
    response_err_count: usize,
    sum_request_durations: Duration,
    devices_diag: HashMap<String, StoreDeviceDiag>,
}

impl StoreFieldbusDiag {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            response_ok_count: 0,
            response_err_count: 0,
            sum_request_durations: Duration::default(),
            devices_diag: HashMap::new(),
        }
    }

    pub fn msg_fieldbus_request_ok(&mut self, duration: Duration) {
        self.response_ok_count += 1;
        self.sum_request_durations += duration;
    }

    pub fn msg_fieldbus_request_err(&mut self, duration: Duration) {
        self.response_err_count += 1;
        self.sum_request_durations += duration;
    }

    pub fn msg_device_init_completed(&mut self, id: &str, duration: Duration) {
        match self.devices_diag.get_mut(id) {
            Some(device_diag) => {
                device_diag.msg_device_init_completed(duration);
            }
            None => {
                let mut device_diag = StoreDeviceDiag::new(id.to_string());
                device_diag.msg_device_init_completed(duration);
                self.devices_diag.insert(id.to_string(), device_diag);
            }
        }
    }

    pub fn msg_device_request_ok(&mut self, id: &str, duration: Duration) {
        match self.devices_diag.get_mut(id) {
            Some(device_diag) => {
                device_diag.msg_device_request_ok(duration);
            }
            None => {
                let mut device_diag = StoreDeviceDiag::new(id.to_string());
                device_diag.msg_device_request_ok(duration);
                self.devices_diag.insert(id.to_string(), device_diag);
            }
        }
    }

    pub fn msg_device_request_err(&mut self, id: &str, duration: Duration, err: String) {
        match self.devices_diag.get_mut(id) {
            Some(device_diag) => {
                device_diag.msg_device_request_err(duration, err);
            }
            None => {
                let mut device_diag = StoreDeviceDiag::new(id.to_string());
                device_diag.msg_device_request_err(duration, err);
                self.devices_diag.insert(id.to_string(), device_diag);
            }
        }
    }
}

impl From<&StoreFieldbusDiag> for FieldbusDiag {
    fn from(value: &StoreFieldbusDiag) -> Self {
        let device_diag = value
            .devices_diag
            .iter()
            .map(|(k, v)| (k.clone(), v.into()))
            .collect::<HashMap<String, DeviceDiag>>();

        let usage =
            value.sum_request_durations.as_secs_f64() / value.start_time.elapsed().as_secs_f64();

        Self {
            devices_diag: device_diag,
            response_ok_count: value.response_ok_count,
            response_err_count: value.response_err_count,
            usage: usage as f32,
        }
    }
}
