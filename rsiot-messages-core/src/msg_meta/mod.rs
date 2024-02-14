mod component_id;
mod service_id;
mod timestamp;

pub use component_id::ComponentId;
pub use service_id::ServiceId;
pub use timestamp::Timestamp;

pub trait MsgMeta {
    fn ts(&self) -> Timestamp;

    fn cmp_source(&self) -> Option<ComponentId>;

    fn cmp_process(&self) -> Option<ComponentId>;

    fn cmp_set(&mut self, cmp_id: &ComponentId);

    /// Возвращает поле `value` в заданном формате
    fn fmt_value(&self, template: &str) -> String;
}
