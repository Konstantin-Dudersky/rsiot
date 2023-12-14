use serde::{Deserialize, Serialize};

use rsiot_messages_core::{msg_types, IMessage};

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Messages {
    wWordState1_Get(msg_types::Value<u16>),
    wWordState2(msg_types::Value<u16>),
    wWordCMD1(msg_types::Value<u16>),
    iOperationModeSet(msg_types::Value<u16>),
    rTempCSSensorTValue_Get(msg_types::Value<f32>),
    rMZSensorCurrentRS1_Get(msg_types::Value<f32>),
    rMZSensorCurrentRS2_Get(msg_types::Value<f32>),
    rFZSensorCurrentRS3_Get(msg_types::Value<f32>),
    rCZSensorCurrentRS4_Get(msg_types::Value<f32>),
    BZTimeBetweenStartManual(msg_types::Value<u16>),
    BZTimePulseStartManual(msg_types::Value<u16>),
    CZTimeBetweenStartManual(msg_types::Value<u16>),
    CZTimePulseStartManual(msg_types::Value<u16>),
    TimeFermentH_Current(msg_types::Value<u16>),
    wWordError(msg_types::Value<u16>),
    Invertor20Frequency(msg_types::Value<u16>),
    Invertor22Frequency(msg_types::Value<u16>),
    Invertor24Frequency(msg_types::Value<u16>),
    TimeFermentM_Current(msg_types::Value<u16>),
    motodays(msg_types::Value<u32>),
    motohours(msg_types::Value<u32>),
    motomins(msg_types::Value<u32>),
    CZDispenserL(msg_types::Value<f32>),
    FZMixerPart1N(msg_types::Value<u16>),
    FZMixerPart2N(msg_types::Value<u16>),
    FZMixerPart3N(msg_types::Value<u16>),
    FZMixerPart4N(msg_types::Value<u16>),
    MZMixerPart1N(msg_types::Value<u16>),
    MZMixerPart2N(msg_types::Value<u16>),
    MZMixerPart3N(msg_types::Value<u16>),
    MZMixerPart4N(msg_types::Value<u16>),
    TimeFermentH_SP(msg_types::Value<u16>),
    TimeFermentM_SP(msg_types::Value<u16>),
    NumberFormation_Get(msg_types::Value<u16>),
    AllMixingSec(msg_types::Value<u16>),
    CZDispenserH(msg_types::Value<u16>),
    CZDispenserM(msg_types::Value<u16>),
    NumberRecept(msg_types::Value<u16>),
    CommError(msg_types::Value<u16>),
    BZDispenserH(msg_types::Value<u16>),
    BZDispenserM(msg_types::Value<u16>),
    BZDispenserL(msg_types::Value<f32>),
}

impl IMessage for Messages {
    fn into_eav(self) -> Vec<rsiot_messages_core::eav::EavModel> {
        vec![]
    }
}
