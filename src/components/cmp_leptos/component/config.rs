use leptos::prelude::*;
use reactive_stores::Store;
use tokio::sync::mpsc;

use crate::message::{Message, MsgDataBound};

use super::StoreBound;

// ANCHOR: Config
/// Конфигурация компонента cmp_leptos
pub struct Config<TMsg, TInputStore, TOutputStore>
where
    TMsg: MsgDataBound,
    TInputStore: StoreBound,
    TOutputStore: StoreBound,
{
    /// Корневой компонент для монтирования
    ///
    /// **Примеры**
    ///
    /// ```rust
    /// body_component: || view! { <app::App/> }.into_any()
    /// ```
    pub body_component: fn() -> AnyView,

    /// Значения по-умолчанию глобального хранилища входных данных
    pub input_store: TInputStore,

    /// Значения по-умолчанию глобального хранилища выходных данных
    pub output_store: TOutputStore,

    /// Функция обрабатывает входящие сообщения и сохраняет в хранилище Store
    pub fn_input: fn(&Message<TMsg>, &Store<TInputStore>),

    /// Функция проверяет изменения в хранилищe Store и отправляет исходящие сообщения
    pub fn_output: fn(Store<TOutputStore>, mpsc::Sender<TMsg>),
}
// ANCHOR: Config
