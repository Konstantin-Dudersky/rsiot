use std::time::Duration;

use crate::{
    components_config::can_general::{BufferBound, CanFilter, CanFrame, CanSettings},
    message::MsgDataBound,
};

// ANCHOR: Config
/// Конфигурация компонента cmp_linux_can
#[derive(Clone)]
pub struct Config<TMsg, TBuffer>
where
    TMsg: MsgDataBound,
    TBuffer: BufferBound,
{
    /// Интерфейс CAN, например "vcan0"
    pub ifname: String,

    /// Задание режимов CAN-протокола
    pub can_settings: CanSettings,

    /// Значение в буфере по умолчанию.
    ///
    /// Буфер используется для отправки периодических сообщений.
    ///
    /// Если буфер не используется, можно задать значение `()`.
    pub buffer_default: TBuffer,

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
    pub fn_input: fn(&TMsg, &mut TBuffer) -> anyhow::Result<Option<Vec<CanFrame>>>,

    /// Функция периодического создания кадров
    pub fn_periodic: fn(&TBuffer) -> anyhow::Result<Option<Vec<CanFrame>>>,

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
    pub fn_output: fn(CanFrame) -> Option<Vec<TMsg>>,

    /// true - асинхронная версия
    ///
    /// Синхронная версия запускается в отдельных потоках, производительность в ~5 раз выше
    pub async_version: bool,
}
// ANCHOR: Config
