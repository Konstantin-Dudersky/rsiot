use std::time::Duration;

use esp_idf_svc::hal::{can::CAN, gpio::AnyIOPin};

use crate::{
    components_config::can_general::{CanFilter, CanFrame, CanSettings},
    message::MsgDataBound,
};

/// Конфигурация компонента cmp_esp_can
pub struct Config<TMsg, TFnInput>
where
    TMsg: MsgDataBound,
    TFnInput: Fn(&TMsg) -> anyhow::Result<Option<Vec<CanFrame>>>,
{
    /// Ссылка на аппаратный интерфейс CAN
    pub can: CAN<'static>,

    /// Пин сигнала TX
    pub pin_tx: AnyIOPin<'static>,

    /// Пин сигнала RX
    pub pin_rx: AnyIOPin<'static>,

    /// Задание режимов CAN-протокола
    pub can_settings: CanSettings,

    /// Преобразование входящих сообщений в CAN-сообщения
    ///
    /// # Примеры
    ///
    /// ## Задать f32
    ///
    /// ```rs
    /// let mut data = [0u8; 8];
    /// data[4..8].copy_from_slice(&f.to_be_bytes());
    /// ```
    ///
    /// ## Задать u16
    ///
    /// ```rs
    /// let mut data = [0u8; 8];
    /// let bits = data.view_bits_mut::<Msb0>();
    /// bits[32..48].store_be(*v);
    /// ```
    pub fn_input: TFnInput,

    /// Период создания кадров
    pub period: Duration,

    /// Настройка фильтрации получаемых CAN-сообщений
    ///
    /// Пустой вектор означает приём всех сообщений
    ///
    /// Приём от источника с адресом 1:
    /// - id: 0x00_01_00_00
    /// - mask: 0x00_FF_00_00
    ///
    /// Приём для адресата с адресом 1:
    /// - id: 0x00_00_01_00
    /// - mask: 0x00_00_FF_00
    pub filters: Vec<CanFilter>,

    /// Преобразование полученного CAN-сообщения в исходящие сообщения
    ///
    /// # Примеры
    ///
    /// ## Загрузить u8
    /// ```rs
    /// let bits = data.view_bits::<Lsb0>();
    /// let value = bits[0..8].load_be::<u8>();
    /// let msg = Msg::CanDataFromBus(value);
    /// ```
    pub fn_output: fn(CanFrame) -> anyhow::Result<Option<Vec<TMsg>>>,
}
