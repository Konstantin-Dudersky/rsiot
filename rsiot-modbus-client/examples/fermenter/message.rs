use serde::{Deserialize, Serialize};

use rsiot_messages_core::{msg_meta, IMessage, MsgContent, MsgMeta};

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, MsgMeta)]
pub enum Messages {
    wWordState1_Get(MsgContent<u16>),
    wWordState2(MsgContent<u16>),
    wWordCMD1(MsgContent<u16>),
    iOperationModeSet(MsgContent<u16>),
    rTempCSSensorTValue_Get(MsgContent<f32>),
    rMZSensorCurrentRS1_Get(MsgContent<f32>),
    rMZSensorCurrentRS2_Get(MsgContent<f32>),
    rFZSensorCurrentRS3_Get(MsgContent<f32>),
    rCZSensorCurrentRS4_Get(MsgContent<f32>),
    BZTimeBetweenStartManual(MsgContent<u16>),
    BZTimePulseStartManual(MsgContent<u16>),
    CZTimeBetweenStartManual(MsgContent<u16>),
    CZTimePulseStartManual(MsgContent<u16>),
    TimeFermentH_Current(MsgContent<u16>),
    wWordError(MsgContent<u16>),
    Invertor20Frequency(MsgContent<u16>),
    Invertor22Frequency(MsgContent<u16>),
    Invertor24Frequency(MsgContent<u16>),
    TimeFermentM_Current(MsgContent<u16>),
    motodays(MsgContent<u32>),
    motohours(MsgContent<u32>),
    motomins(MsgContent<u32>),
    CZDispenserL(MsgContent<f32>),
    FZMixerPart1N(MsgContent<u16>),
    FZMixerPart2N(MsgContent<u16>),
    FZMixerPart3N(MsgContent<u16>),
    FZMixerPart4N(MsgContent<u16>),
    MZMixerPart1N(MsgContent<u16>),
    MZMixerPart2N(MsgContent<u16>),
    MZMixerPart3N(MsgContent<u16>),
    MZMixerPart4N(MsgContent<u16>),
    TimeFermentH_SP(MsgContent<u16>),
    TimeFermentM_SP(MsgContent<u16>),
    NumberFormation_Get(MsgContent<u16>),
    AllMixingSec(MsgContent<u16>),
    CZDispenserH(MsgContent<u16>),
    CZDispenserM(MsgContent<u16>),
    NumberRecept(MsgContent<u16>),
    CommError(MsgContent<u16>),
    BZDispenserH(MsgContent<u16>),
    BZDispenserM(MsgContent<u16>),
    BZDispenserL(MsgContent<f32>),
}

impl IMessage for Messages {
    fn into_eav(self) -> Vec<rsiot_messages_core::eav::EavModel> {
        vec![]
    }
}
