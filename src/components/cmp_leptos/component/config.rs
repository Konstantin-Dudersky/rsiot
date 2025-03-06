use leptos::prelude::*;
use reactive_stores::Store;
use tokio::sync::mpsc;

use crate::message::{Message, MsgDataBound};

use super::StoreBound;

/// Конфигурация компонента cmp_leptos
pub struct Config<TMsg, TView, TIntoView, TInputStore, TOutputStore>
where
    TMsg: MsgDataBound,
    TView: Fn() -> TIntoView,
    TIntoView: IntoView,
    TInputStore: StoreBound,
    TOutputStore: StoreBound,
{
    /// Корневой компонент для монтирования
    ///
    /// **Примеры**
    ///
    /// ```rust
    /// body_component: || view! { <App/> }
    /// ```
    pub body_component: TView,

    /// Значения по-умолчанию глобального хранилища входных данных
    pub input_store: TInputStore,

    /// Значения по-умолчанию глобального хранилища выходных данных
    pub output_store: TOutputStore,

    /// Функция обрабатывает входящие сообщения и сохраняет в хранилище Store
    pub fn_input: fn(&Message<TMsg>, &Store<TInputStore>),

    /// Функция проверяет изменения в хранилищe Store и отправляет исходящие сообщения
    pub fn_output: fn(Store<TOutputStore>, mpsc::Sender<TMsg>),
}
