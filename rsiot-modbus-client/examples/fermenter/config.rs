use std::net::{IpAddr, Ipv4Addr};

use tokio::time::Duration;

use rsiot_messages_core::MsgContent;
use rsiot_modbus_client::cmp_modbus_client::{self, *};

use super::message::Messages;

// Псеводнимы для функций преобразования данных
const TO_F32: fn(&[u16]) -> f32 = conversion::to_f32::little_endian_swap;
const TO_U32: fn(&[u16]) -> u32 = conversion::to_u32::little_endian_swap;

/// Конфигурация modbus клиента
pub fn config() -> Config<Messages> {
    cmp_modbus_client::Config {
        enabled: true,
        unit_id: 1,
        input_config: vec![InputConfig {
            fn_input: |_| None,
            fn_on_success: |_| vec![],
            fn_on_failure: Vec::new,
        }],
        periodic_config: vec![PeriodicConfig {
            period: Duration::from_secs(2),
            request: Request::ReadHoldingRegisters(0, 52),
            fn_on_success: fn_on_success_1,
            fn_on_failure: || vec![Messages::rTempCSSensorTValue_Get(MsgContent::new(0.0))],
        }],
        client_type: ClientType::Tcp(TcpClientType {
            host: IpAddr::V4(Ipv4Addr::new(10, 0, 6, 10)),
            port: 502,
        }),
    }
}

#[allow(non_snake_case)]
fn fn_on_success_1(data: &Response) -> Vec<Messages> {
    let data = match data {
        Response::U16(data) => data,
        _ => return vec![],
    };
    let wWordState1_Get = data[0];
    let wWordState1_Get = Messages::wWordState1_Get(MsgContent::new(wWordState1_Get));

    let wWordState2 = data[1];
    let wWordState2 = Messages::wWordState2(MsgContent::new(wWordState2));

    let wWordCMD1 = data[2];
    let wWordCMD1 = Messages::wWordCMD1(MsgContent::new(wWordCMD1));

    let iOperationModeSet = data[3];
    let iOperationModeSet = Messages::iOperationModeSet(MsgContent::new(iOperationModeSet));

    let rTempCSSensorTValue_Get = TO_F32(&data[4..=5]);
    let rTempCSSensorTValue_Get =
        Messages::rTempCSSensorTValue_Get(MsgContent::new(rTempCSSensorTValue_Get));

    let rMZSensorCurrentRS1_Get = TO_F32(&data[6..=7]);
    let rMZSensorCurrentRS1_Get =
        Messages::rMZSensorCurrentRS1_Get(MsgContent::new(rMZSensorCurrentRS1_Get));

    let rMZSensorCurrentRS2_Get = TO_F32(&data[8..=9]);
    let rMZSensorCurrentRS2_Get =
        Messages::rMZSensorCurrentRS2_Get(MsgContent::new(rMZSensorCurrentRS2_Get));

    let rFZSensorCurrentRS3_Get = TO_F32(&data[10..=11]);
    let rFZSensorCurrentRS3_Get =
        Messages::rFZSensorCurrentRS3_Get(MsgContent::new(rFZSensorCurrentRS3_Get));

    let rCZSensorCurrentRS4_Get = TO_F32(&data[12..=13]);
    let rCZSensorCurrentRS4_Get =
        Messages::rCZSensorCurrentRS4_Get(MsgContent::new(rCZSensorCurrentRS4_Get));

    let BZTimeBetweenStartManual = data[14];
    let BZTimeBetweenStartManual =
        Messages::BZTimeBetweenStartManual(MsgContent::new(BZTimeBetweenStartManual));

    let BZTimePulseStartManual = data[15];
    let BZTimePulseStartManual =
        Messages::BZTimePulseStartManual(MsgContent::new(BZTimePulseStartManual));

    let CZTimeBetweenStartManual = data[16];
    let CZTimeBetweenStartManual =
        Messages::CZTimeBetweenStartManual(MsgContent::new(CZTimeBetweenStartManual));

    let CZTimePulseStartManual = data[17];
    let CZTimePulseStartManual =
        Messages::CZTimePulseStartManual(MsgContent::new(CZTimePulseStartManual));

    let TimeFermentH_Current = data[18];
    let TimeFermentH_Current =
        Messages::TimeFermentH_Current(MsgContent::new(TimeFermentH_Current));

    let wWordError = data[19];
    let wWordError = Messages::wWordError(MsgContent::new(wWordError));

    let Invertor20Frequency = data[20];
    let Invertor20Frequency = Messages::Invertor20Frequency(MsgContent::new(Invertor20Frequency));

    let Invertor22Frequency = data[21];
    let Invertor22Frequency = Messages::Invertor22Frequency(MsgContent::new(Invertor22Frequency));

    let Invertor24Frequency = data[22];
    let Invertor24Frequency = Messages::Invertor24Frequency(MsgContent::new(Invertor24Frequency));

    let TimeFermentM_Current = data[23];
    let TimeFermentM_Current =
        Messages::TimeFermentM_Current(MsgContent::new(TimeFermentM_Current));

    let motodays = TO_U32(&data[24..=25]);
    let motodays = Messages::motodays(MsgContent::new(motodays));

    let motohours = TO_U32(&data[26..=27]);
    let motohours = Messages::motohours(MsgContent::new(motohours));

    let motomins = TO_U32(&data[28..=29]);
    let motomins = Messages::motomins(MsgContent::new(motomins));

    let CZDispenserL = TO_F32(&data[30..=31]);
    let CZDispenserL = Messages::CZDispenserL(MsgContent::new(CZDispenserL));

    let FZMixerPart1N = data[32];
    let FZMixerPart1N = Messages::FZMixerPart1N(MsgContent::new(FZMixerPart1N));

    let FZMixerPart2N = data[33];
    let FZMixerPart2N = Messages::FZMixerPart2N(MsgContent::new(FZMixerPart2N));

    let FZMixerPart3N = data[34];
    let FZMixerPart3N = Messages::FZMixerPart3N(MsgContent::new(FZMixerPart3N));

    let FZMixerPart4N = data[35];
    let FZMixerPart4N = Messages::FZMixerPart4N(MsgContent::new(FZMixerPart4N));

    let MZMixerPart1N = data[38];
    let MZMixerPart1N = Messages::MZMixerPart1N(MsgContent::new(MZMixerPart1N));

    let MZMixerPart2N = data[37];
    let MZMixerPart2N = Messages::MZMixerPart2N(MsgContent::new(MZMixerPart2N));

    let MZMixerPart3N = data[38];
    let MZMixerPart3N = Messages::MZMixerPart3N(MsgContent::new(MZMixerPart3N));

    let MZMixerPart4N = data[39];
    let MZMixerPart4N = Messages::MZMixerPart4N(MsgContent::new(MZMixerPart4N));

    let TimeFermentH_SP = data[40];
    let TimeFermentH_SP = Messages::TimeFermentH_SP(MsgContent::new(TimeFermentH_SP));

    let TimeFermentM_SP = data[41];
    let TimeFermentM_SP = Messages::TimeFermentM_SP(MsgContent::new(TimeFermentM_SP));

    let NumberFormation_Get = data[42];
    let NumberFormation_Get = Messages::NumberFormation_Get(MsgContent::new(NumberFormation_Get));

    let AllMixingSec = data[43];
    let AllMixingSec = Messages::AllMixingSec(MsgContent::new(AllMixingSec));

    let CZDispenserH = data[44];
    let CZDispenserH = Messages::CZDispenserH(MsgContent::new(CZDispenserH));

    let CZDispenserM = data[45];
    let CZDispenserM = Messages::CZDispenserM(MsgContent::new(CZDispenserM));

    let NumberRecept = data[46];
    let NumberRecept = Messages::NumberRecept(MsgContent::new(NumberRecept));

    let CommError = data[47];
    let CommError = Messages::CommError(MsgContent::new(CommError));

    let BZDispenserH = data[48];
    let BZDispenserH = Messages::BZDispenserH(MsgContent::new(BZDispenserH));

    let BZDispenserM = data[49];
    let BZDispenserM = Messages::BZDispenserM(MsgContent::new(BZDispenserM));

    let BZDispenserL = TO_F32(&data[50..=51]);
    let BZDispenserL = Messages::BZDispenserL(MsgContent::new(BZDispenserL));

    vec![
        wWordState1_Get,
        wWordState2,
        wWordCMD1,
        iOperationModeSet,
        rTempCSSensorTValue_Get,
        rMZSensorCurrentRS1_Get,
        rMZSensorCurrentRS2_Get,
        rFZSensorCurrentRS3_Get,
        rCZSensorCurrentRS4_Get,
        BZTimeBetweenStartManual,
        BZTimePulseStartManual,
        CZTimeBetweenStartManual,
        CZTimePulseStartManual,
        TimeFermentH_Current,
        wWordError,
        Invertor20Frequency,
        Invertor22Frequency,
        Invertor24Frequency,
        TimeFermentM_Current,
        motodays,
        motohours,
        motomins,
        CZDispenserL,
        FZMixerPart1N,
        FZMixerPart2N,
        FZMixerPart3N,
        FZMixerPart4N,
        MZMixerPart1N,
        MZMixerPart2N,
        MZMixerPart3N,
        MZMixerPart4N,
        TimeFermentH_SP,
        TimeFermentM_SP,
        NumberFormation_Get,
        AllMixingSec,
        CZDispenserH,
        CZDispenserM,
        NumberRecept,
        CommError,
        BZDispenserH,
        BZDispenserM,
        BZDispenserL,
    ]
}
