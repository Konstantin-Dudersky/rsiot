//! Исполнение логики работы в стиле ПЛК.
//!
//! ## Структурные единицы
//!
//! - Функциональные блоки (FB)
//!
//! ### Организационные блоки (OB)
//!
//! Верхнеуровневые элементы. Выполнены в виде задач tokio. Параллельно можно запускать несколько
//! OB. Поскольку задачи выполняются в разных потоках, нет необходимости вытеснять OB по приоритетам
//! - они выполняются параллельно.
//!
//! plc-rs не заботится, откуда данные приходят и куда отправляются. Входные данные поступают из
//! брокера сообщений и передаются по каналу в OB. Выходные данные также отправляются в брокер
//! сообщений. Коммуникации с устройствами также программируются во внешних крейтах.
//!
//! ### Функциональные блоки (FB)
//!
//! Базовые компоненты для построения программы. Сохраняют состояние между вызовами.
//!

mod component;
mod config;
mod fn_process;
pub mod plc;
mod template;

/// Компонент для исполнения логики наподобие PLC.
///
/// См. [документацию](https://docs.rs/rsiot-plc/latest/)
pub mod cmp_plc {
    pub use crate::component::Cmp;
    pub use crate::config::Config;
    pub use crate::plc;
}
