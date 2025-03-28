use std::fmt::Debug;

use serde::{de::DeserializeOwned, Serialize};

use crate::{message::MsgDataBound, serde_utils::SerdeAlgKind};

/// Функция преобразования сообщений в текстовые файлы.
///
/// Возращает кортеж из двух значений:
/// - название файла для сохранения
/// - содержимое файла
pub type FnInput<TMsg, TBuffer> = fn(&TMsg, &TBuffer) -> Option<TBuffer>;

/// Функция преобразования текстовых файлов в сообщения
pub type FnOutput<TMsg, TBuffer> = fn(&TBuffer) -> Vec<TMsg>;

/// Конфигурация cmp_filesystem
#[derive(Clone)]
pub struct Config<TMsg, TBuffer>
where
    TMsg: MsgDataBound,
    TBuffer: BufferBound,
{
    /// Алгоритм сериализации/десериализации
    pub serde_alg: SerdeAlgKind,

    /// Папка, в которой хранятся файлы
    pub filename: String,

    /// Частота вызова функции создания исходящих сообщений
    pub call_fn_output_kind: CallFnOutputKind,

    /// Функция преобразования сообщений в текстовые файлы
    pub fn_input: FnInput<TMsg, TBuffer>,

    /// Функция преобразования текстовых файлов в сообщения
    pub fn_output: FnOutput<TMsg, TBuffer>,
}

/// Частота вызова функции создания исходящих сообщений
#[derive(Clone, Debug)]
pub enum CallFnOutputKind {
    /// Один раз при запуске
    OnStartup,
    /// Каждый раз при изменении буфера
    Always,
}

/// Ограничения на структуру буфера
///
/// На структуре необходимо релизовать:
///
/// ```no_run
/// #[derive(Clone, Debug, Default, Deserialize, Serialize)]
/// ```
pub trait BufferBound:
    Clone + Debug + Default + DeserializeOwned + Send + Serialize + Sync
{
}
