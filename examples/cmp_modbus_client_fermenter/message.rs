use serde::{Deserialize, Serialize};

use rsiot::message::{MsgDataBound, MsgKey};

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
pub enum Data {
    wWordState1_Get(u16),
    wWordState2(u16),
    wWordCMD1(u16),
    iOperationModeSet(u16),
    rTempCSSensorTValue_Get(f32),
    rMZSensorCurrentRS1_Get(f32),
    rMZSensorCurrentRS2_Get(f32),
    rFZSensorCurrentRS3_Get(f32),
    rCZSensorCurrentRS4_Get(f32),
    BZTimeBetweenStartManual(u16),
    BZTimePulseStartManual(u16),
    CZTimeBetweenStartManual(u16),
    CZTimePulseStartManual(u16),
    TimeFermentH_Current(u16),
    wWordError(u16),
    Invertor20Frequency(u16),
    Invertor22Frequency(u16),
    Invertor24Frequency(u16),
    TimeFermentM_Current(u16),
    motodays(u32),
    motohours(u32),
    motomins(u32),
    CZDispenserL(f32),
    FZMixerPart1N(u16),
    FZMixerPart2N(u16),
    FZMixerPart3N(u16),
    FZMixerPart4N(u16),
    MZMixerPart1N(u16),
    MZMixerPart2N(u16),
    MZMixerPart3N(u16),
    MZMixerPart4N(u16),
    TimeFermentH_SP(u16),
    TimeFermentM_SP(u16),
    NumberFormation_Get(u16),
    AllMixingSec(u16),
    CZDispenserH(u16),
    CZDispenserM(u16),
    NumberRecept(u16),
    CommError(u16),
    BZDispenserH(u16),
    BZDispenserM(u16),
    BZDispenserL(f32),
}

impl MsgDataBound for Data {}
