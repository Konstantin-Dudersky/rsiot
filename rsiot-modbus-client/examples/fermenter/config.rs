use std::net::{IpAddr, Ipv4Addr};

use tokio::time::Duration;

use rsiot_messages_core::Message;
use rsiot_modbus_client::cmp_modbus_client::{self, *};

use super::message::Data;

// Псеводнимы для функций преобразования данных
const TO_F32: fn(&[u16]) -> f32 = conversion::to_f32::little_endian_swap;
const TO_U32: fn(&[u16]) -> u32 = conversion::to_u32::little_endian_swap;

/// Конфигурация modbus клиента
pub fn config() -> Config<Data> {
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
            fn_on_failure: || vec![Message::new(Data::rTempCSSensorTValue_Get(0.0))],
        }],
        client_type: ClientType::Tcp(TcpClientType {
            host: IpAddr::V4(Ipv4Addr::new(10, 0, 6, 10)),
            port: 502,
        }),
    }
}

#[allow(non_snake_case)]
fn fn_on_success_1(data: &Response) -> Vec<Message<Data>> {
    let data = match data {
        Response::U16(data) => data,
        _ => return vec![],
    };
    let wWordState1_Get = data[0];
    let wWordState1_Get = Message::new(Data::wWordState1_Get(wWordState1_Get));

    let wWordState2 = data[1];
    let wWordState2 = Message::new(Data::wWordState2(wWordState2));

    let wWordCMD1 = data[2];
    let wWordCMD1 = Message::new(Data::wWordCMD1(wWordCMD1));

    let iOperationModeSet = data[3];
    let iOperationModeSet = Message::new(Data::iOperationModeSet(iOperationModeSet));

    let rTempCSSensorTValue_Get = TO_F32(&data[4..=5]);
    let rTempCSSensorTValue_Get =
        Message::new(Data::rTempCSSensorTValue_Get(rTempCSSensorTValue_Get));

    let rMZSensorCurrentRS1_Get = TO_F32(&data[6..=7]);
    let rMZSensorCurrentRS1_Get =
        Message::new(Data::rMZSensorCurrentRS1_Get(rMZSensorCurrentRS1_Get));

    let rMZSensorCurrentRS2_Get = TO_F32(&data[8..=9]);
    let rMZSensorCurrentRS2_Get =
        Message::new(Data::rMZSensorCurrentRS2_Get(rMZSensorCurrentRS2_Get));

    let rFZSensorCurrentRS3_Get = TO_F32(&data[10..=11]);
    let rFZSensorCurrentRS3_Get =
        Message::new(Data::rFZSensorCurrentRS3_Get(rFZSensorCurrentRS3_Get));

    let rCZSensorCurrentRS4_Get = TO_F32(&data[12..=13]);
    let rCZSensorCurrentRS4_Get =
        Message::new(Data::rCZSensorCurrentRS4_Get(rCZSensorCurrentRS4_Get));

    let BZTimeBetweenStartManual = data[14];
    let BZTimeBetweenStartManual =
        Message::new(Data::BZTimeBetweenStartManual(BZTimeBetweenStartManual));

    let BZTimePulseStartManual = data[15];
    let BZTimePulseStartManual = Message::new(Data::BZTimePulseStartManual(BZTimePulseStartManual));

    let CZTimeBetweenStartManual = data[16];
    let CZTimeBetweenStartManual =
        Message::new(Data::CZTimeBetweenStartManual(CZTimeBetweenStartManual));

    let CZTimePulseStartManual = data[17];
    let CZTimePulseStartManual = Message::new(Data::CZTimePulseStartManual(CZTimePulseStartManual));

    let TimeFermentH_Current = data[18];
    let TimeFermentH_Current = Message::new(Data::TimeFermentH_Current(TimeFermentH_Current));

    let wWordError = data[19];
    let wWordError = Message::new(Data::wWordError(wWordError));

    let Invertor20Frequency = data[20];
    let Invertor20Frequency = Message::new(Data::Invertor20Frequency(Invertor20Frequency));

    let Invertor22Frequency = data[21];
    let Invertor22Frequency = Message::new(Data::Invertor22Frequency(Invertor22Frequency));

    let Invertor24Frequency = data[22];
    let Invertor24Frequency = Message::new(Data::Invertor24Frequency(Invertor24Frequency));

    let TimeFermentM_Current = data[23];
    let TimeFermentM_Current = Message::new(Data::TimeFermentM_Current(TimeFermentM_Current));

    let motodays = TO_U32(&data[24..=25]);
    let motodays = Message::new(Data::motodays(motodays));

    let motohours = TO_U32(&data[26..=27]);
    let motohours = Message::new(Data::motohours(motohours));

    let motomins = TO_U32(&data[28..=29]);
    let motomins = Message::new(Data::motomins(motomins));

    let CZDispenserL = TO_F32(&data[30..=31]);
    let CZDispenserL = Message::new(Data::CZDispenserL(CZDispenserL));

    let FZMixerPart1N = data[32];
    let FZMixerPart1N = Message::new(Data::FZMixerPart1N(FZMixerPart1N));

    let FZMixerPart2N = data[33];
    let FZMixerPart2N = Message::new(Data::FZMixerPart2N(FZMixerPart2N));

    let FZMixerPart3N = data[34];
    let FZMixerPart3N = Message::new(Data::FZMixerPart3N(FZMixerPart3N));

    let FZMixerPart4N = data[35];
    let FZMixerPart4N = Message::new(Data::FZMixerPart4N(FZMixerPart4N));

    let MZMixerPart1N = data[38];
    let MZMixerPart1N = Message::new(Data::MZMixerPart1N(MZMixerPart1N));

    let MZMixerPart2N = data[37];
    let MZMixerPart2N = Message::new(Data::MZMixerPart2N(MZMixerPart2N));

    let MZMixerPart3N = data[38];
    let MZMixerPart3N = Message::new(Data::MZMixerPart3N(MZMixerPart3N));

    let MZMixerPart4N = data[39];
    let MZMixerPart4N = Message::new(Data::MZMixerPart4N(MZMixerPart4N));

    let TimeFermentH_SP = data[40];
    let TimeFermentH_SP = Message::new(Data::TimeFermentH_SP(TimeFermentH_SP));

    let TimeFermentM_SP = data[41];
    let TimeFermentM_SP = Message::new(Data::TimeFermentM_SP(TimeFermentM_SP));

    let NumberFormation_Get = data[42];
    let NumberFormation_Get = Message::new(Data::NumberFormation_Get(NumberFormation_Get));

    let AllMixingSec = data[43];
    let AllMixingSec = Message::new(Data::AllMixingSec(AllMixingSec));

    let CZDispenserH = data[44];
    let CZDispenserH = Message::new(Data::CZDispenserH(CZDispenserH));

    let CZDispenserM = data[45];
    let CZDispenserM = Message::new(Data::CZDispenserM(CZDispenserM));

    let NumberRecept = data[46];
    let NumberRecept = Message::new(Data::NumberRecept(NumberRecept));

    let CommError = data[47];
    let CommError = Message::new(Data::CommError(CommError));

    let BZDispenserH = data[48];
    let BZDispenserH = Message::new(Data::BZDispenserH(BZDispenserH));

    let BZDispenserM = data[49];
    let BZDispenserM = Message::new(Data::BZDispenserM(BZDispenserM));

    let BZDispenserL = TO_F32(&data[50..=51]);
    let BZDispenserL = Message::new(Data::BZDispenserL(BZDispenserL));

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
