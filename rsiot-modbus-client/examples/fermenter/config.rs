use std::net::{IpAddr, Ipv4Addr};

use tokio::time::Duration;

use rsiot_messages_core::msg_types;
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
            fn_on_failure: || {
                vec![Messages::rTempCSSensorTValue_Get(msg_types::Value::new(
                    0.0,
                ))]
            },
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
    let wWordState1_Get = Messages::wWordState1_Get(msg_types::Value::new(wWordState1_Get));

    let wWordState2 = data[1];
    let wWordState2 = Messages::wWordState2(msg_types::Value::new(wWordState2));

    let wWordCMD1 = data[2];
    let wWordCMD1 = Messages::wWordCMD1(msg_types::Value::new(wWordCMD1));

    let iOperationModeSet = data[3];
    let iOperationModeSet = Messages::iOperationModeSet(msg_types::Value::new(iOperationModeSet));

    let rTempCSSensorTValue_Get = TO_F32(&data[4..=5]);
    let rTempCSSensorTValue_Get =
        Messages::rTempCSSensorTValue_Get(msg_types::Value::new(rTempCSSensorTValue_Get));

    let rMZSensorCurrentRS1_Get = TO_F32(&data[6..=7]);
    let rMZSensorCurrentRS1_Get =
        Messages::rMZSensorCurrentRS1_Get(msg_types::Value::new(rMZSensorCurrentRS1_Get));

    let rMZSensorCurrentRS2_Get = TO_F32(&data[8..=9]);
    let rMZSensorCurrentRS2_Get =
        Messages::rMZSensorCurrentRS2_Get(msg_types::Value::new(rMZSensorCurrentRS2_Get));

    let rFZSensorCurrentRS3_Get = TO_F32(&data[10..=11]);
    let rFZSensorCurrentRS3_Get =
        Messages::rFZSensorCurrentRS3_Get(msg_types::Value::new(rFZSensorCurrentRS3_Get));

    let rCZSensorCurrentRS4_Get = TO_F32(&data[12..=13]);
    let rCZSensorCurrentRS4_Get =
        Messages::rCZSensorCurrentRS4_Get(msg_types::Value::new(rCZSensorCurrentRS4_Get));

    let BZTimeBetweenStartManual = data[14];
    let BZTimeBetweenStartManual =
        Messages::BZTimeBetweenStartManual(msg_types::Value::new(BZTimeBetweenStartManual));

    let BZTimePulseStartManual = data[15];
    let BZTimePulseStartManual =
        Messages::BZTimePulseStartManual(msg_types::Value::new(BZTimePulseStartManual));

    let CZTimeBetweenStartManual = data[16];
    let CZTimeBetweenStartManual =
        Messages::CZTimeBetweenStartManual(msg_types::Value::new(CZTimeBetweenStartManual));

    let CZTimePulseStartManual = data[17];
    let CZTimePulseStartManual =
        Messages::CZTimePulseStartManual(msg_types::Value::new(CZTimePulseStartManual));

    let TimeFermentH_Current = data[18];
    let TimeFermentH_Current =
        Messages::TimeFermentH_Current(msg_types::Value::new(TimeFermentH_Current));

    let wWordError = data[19];
    let wWordError = Messages::wWordError(msg_types::Value::new(wWordError));

    let Invertor20Frequency = data[20];
    let Invertor20Frequency =
        Messages::Invertor20Frequency(msg_types::Value::new(Invertor20Frequency));

    let Invertor22Frequency = data[21];
    let Invertor22Frequency =
        Messages::Invertor22Frequency(msg_types::Value::new(Invertor22Frequency));

    let Invertor24Frequency = data[22];
    let Invertor24Frequency =
        Messages::Invertor24Frequency(msg_types::Value::new(Invertor24Frequency));

    let TimeFermentM_Current = data[23];
    let TimeFermentM_Current =
        Messages::TimeFermentM_Current(msg_types::Value::new(TimeFermentM_Current));

    let motodays = TO_U32(&data[24..=25]);
    let motodays = Messages::motodays(msg_types::Value::new(motodays));

    let motohours = TO_U32(&data[26..=27]);
    let motohours = Messages::motohours(msg_types::Value::new(motohours));

    let motomins = TO_U32(&data[28..=29]);
    let motomins = Messages::motomins(msg_types::Value::new(motomins));

    let CZDispenserL = TO_F32(&data[30..=31]);
    let CZDispenserL = Messages::CZDispenserL(msg_types::Value::new(CZDispenserL));

    let FZMixerPart1N = data[32];
    let FZMixerPart1N = Messages::FZMixerPart1N(msg_types::Value::new(FZMixerPart1N));

    let FZMixerPart2N = data[33];
    let FZMixerPart2N = Messages::FZMixerPart2N(msg_types::Value::new(FZMixerPart2N));

    let FZMixerPart3N = data[34];
    let FZMixerPart3N = Messages::FZMixerPart3N(msg_types::Value::new(FZMixerPart3N));

    let FZMixerPart4N = data[35];
    let FZMixerPart4N = Messages::FZMixerPart4N(msg_types::Value::new(FZMixerPart4N));

    let MZMixerPart1N = data[38];
    let MZMixerPart1N = Messages::MZMixerPart1N(msg_types::Value::new(MZMixerPart1N));

    let MZMixerPart2N = data[37];
    let MZMixerPart2N = Messages::MZMixerPart2N(msg_types::Value::new(MZMixerPart2N));

    let MZMixerPart3N = data[38];
    let MZMixerPart3N = Messages::MZMixerPart3N(msg_types::Value::new(MZMixerPart3N));

    let MZMixerPart4N = data[39];
    let MZMixerPart4N = Messages::MZMixerPart4N(msg_types::Value::new(MZMixerPart4N));

    let TimeFermentH_SP = data[40];
    let TimeFermentH_SP = Messages::TimeFermentH_SP(msg_types::Value::new(TimeFermentH_SP));

    let TimeFermentM_SP = data[41];
    let TimeFermentM_SP = Messages::TimeFermentM_SP(msg_types::Value::new(TimeFermentM_SP));

    let NumberFormation_Get = data[42];
    let NumberFormation_Get =
        Messages::NumberFormation_Get(msg_types::Value::new(NumberFormation_Get));

    let AllMixingSec = data[43];
    let AllMixingSec = Messages::AllMixingSec(msg_types::Value::new(AllMixingSec));

    let CZDispenserH = data[44];
    let CZDispenserH = Messages::CZDispenserH(msg_types::Value::new(CZDispenserH));

    let CZDispenserM = data[45];
    let CZDispenserM = Messages::CZDispenserM(msg_types::Value::new(CZDispenserM));

    let NumberRecept = data[46];
    let NumberRecept = Messages::NumberRecept(msg_types::Value::new(NumberRecept));

    let CommError = data[47];
    let CommError = Messages::CommError(msg_types::Value::new(CommError));

    let BZDispenserH = data[48];
    let BZDispenserH = Messages::BZDispenserH(msg_types::Value::new(BZDispenserH));

    let BZDispenserM = data[49];
    let BZDispenserM = Messages::BZDispenserM(msg_types::Value::new(BZDispenserM));

    let BZDispenserL = TO_F32(&data[50..=51]);
    let BZDispenserL = Messages::BZDispenserL(msg_types::Value::new(BZDispenserL));

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
