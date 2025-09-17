//! Общие данные для настройки CAN

mod buffer_bound;
mod can_filter;
mod can_frame;
mod can_id;
mod can_settings;

pub use {
    buffer_bound::BufferBound,
    can_filter::CanFilter,
    can_frame::CanFrame,
    can_id::CanId,
    can_settings::{CanSettings, CanSettingsBitrate, CanSettingsDbitrate},
};
