use std::time::Duration;

use crate::{
    components_config::can_general::{BufferBound, Frame},
    message::MsgDataBound,
};

/// Конфигурация компонента cmp_linux_can
#[derive(Clone)]
pub struct Config<TMsg, TBuffer>
where
    TMsg: MsgDataBound,
    TBuffer: BufferBound,
{
    /// Устройство CAN, например "/dev/can0"
    pub dev_can: String,

    pub buffer_default: TBuffer,

    pub fn_input: fn(&TMsg, &mut TBuffer) -> anyhow::Result<Option<Vec<Frame>>>,

    pub period: Duration,

    pub fn_periodic: fn(&TBuffer) -> anyhow::Result<Option<Vec<Frame>>>,

    pub fn_output: fn(Frame) -> Option<Vec<TMsg>>,
}
