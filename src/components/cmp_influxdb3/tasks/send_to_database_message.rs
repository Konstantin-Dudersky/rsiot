use crate::components_config::influxdb3::LineProtocolItem;

pub enum SendToDatabaseMessage {
    LineProtocolItem(LineProtocolItem),
    SendByTimer,
}
