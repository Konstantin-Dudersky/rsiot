mod service_id;
mod timestamp;

pub use service_id::ServiceId;
pub use timestamp::Timestamp;

pub trait MsgMeta {
    fn ts(&self) -> Timestamp;

    fn source(&self) -> ServiceId;

    fn source_set(&mut self, service_id: ServiceId);

    /// Возвращает поле `value` в заданном формате
    fn fmt_value(&self, template: &str) -> String;
}
