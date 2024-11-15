//! Коммуникация через интерфейс uart под ОС Linux
//!
//! ## Для raspberry
//!
//! Чтобы использовать UART0, необходимо деактивировать Bluetooth
//!
//! Отключить блютуз в файле `/boot/firmware/config.txt`, добавить строку:
//!
//! ```
//! dtoverlay=disable-bt
//! ```
//!
//! Деактивировать сервисы:
//!
//! ```bash
//! sudo systemctl disable hciuart.service
//! sudo systemctl disable bluetooth.service
//! ```
//!
//! Перезагрузить систему
//!
//! Теперь в системе будет интерфейс `ttyAMA0`:
//!
//! ```bash
//! ls -l /dev/serial*
//! ```

mod component;
mod config;
pub mod devices;
mod error;
mod fn_process;
mod tasks;

pub use crate::components_config::uart_general::*;
pub use component::Cmp;
pub use config::*;
pub use error::Error;

use crate::serde_utils::postcard_serde;

type Result<T> = std::result::Result<T, Error>;

type UartMessageRaw = [u8; postcard_serde::MESSAGE_LEN];
