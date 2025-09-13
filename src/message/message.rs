#![allow(clippy::module_inception)]

use std::{fmt::Debug, time::Duration};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{MsgData, MsgDataBound, MsgTrace, TimeToLiveValue, Timestamp};

// ANCHOR: Message
/// Сообщение
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Message<TCustom> {
    /// Данные
    pub data: MsgData<TCustom>,
    /// Ключ
    pub key: String,
    /// Метка времени
    pub ts: Timestamp,
    /// Путь, по котором передавалось сообщение
    pub trace: MsgTrace,
    /// Время жизни сообщения
    ttl: TimeToLiveValue,
}
// ANCHOR: Message

impl<TCustom> Message<TCustom>
where
    TCustom: MsgDataBound,
{
    /// Создать новое сообщение
    pub fn new(data: MsgData<TCustom>) -> Self {
        // let key = define_key(&data);
        // let key = super::define_key::define_key(&data);
        let key = data.key();
        let ttl = data.define_time_to_live();
        Self {
            data,
            key,
            ts: Default::default(),
            trace: MsgTrace::default(),
            ttl,
        }
    }

    /// Создать новое сообщение типа `MsgData::Custom`
    pub fn new_custom(custom_data: TCustom) -> Self {
        let data = MsgData::Custom(custom_data);
        // let key = define_key(&data);
        // let key = super::define_key::define_key(&data);
        let key = data.key();
        let ttl = data.define_time_to_live();
        Self {
            data,
            key,
            ts: Default::default(),
            trace: MsgTrace::default(),
            ttl,
        }
    }

    /// Возвращает данные сообщения, если тип сообщения `MsgData::Custom`
    pub fn get_custom_data(&self) -> Option<TCustom> {
        match &self.data {
            MsgData::System(_) => None,
            MsgData::Custom(data) => Some(data.clone()),
        }
    }

    /// Добавить запись пути
    pub fn add_trace_item(&mut self, id: &Uuid) {
        self.trace.add_trace_item(*id)
    }

    /// Проверяем, что в трейсе сообщения присутсвует компонент с заданным id.
    ///
    /// Полезно для предотварщения зацикливания сообщений, чтобы данный компонент не обрабатывал
    /// сообщения, которые он же и сгенерировал
    pub fn contains_trace_item(&self, id: &Uuid) -> bool {
        self.trace.contains_trace_item(id)
    }

    /// Обновить время жизни сообщения
    pub fn update_time_to_live(&mut self, time_step: Duration) {
        match self.ttl {
            TimeToLiveValue::Infinite => (),
            TimeToLiveValue::Duration(duration) => {
                let ttl_new = duration.checked_sub(time_step);
                match ttl_new {
                    Some(ttl_new) => self.ttl = TimeToLiveValue::Duration(ttl_new),
                    None => self.ttl = TimeToLiveValue::Duration(Duration::new(0, 0)),
                }
            }
            TimeToLiveValue::DisableCaching => (),
        }
    }

    /// Возвращает false, если время жизни сообщения истекло
    pub fn is_alive(&self) -> bool {
        match self.ttl {
            TimeToLiveValue::Infinite => true,
            TimeToLiveValue::Duration(duration) => !duration.is_zero(),
            TimeToLiveValue::DisableCaching => false,
        }
    }

    // Разрешен ли марштур данного сообщения
    // pub fn is_route_enabled(&self, src: &TCustom::TService, dst: &TCustom::TService) -> bool {
    //     let route = match &self.data {
    //         MsgData::System(data) => return data.define_enabled_routes(),
    //         MsgData::Custom(data) => data.define_enabled_routes(),
    //     };
    //     match route {
    //         MsgRoute::SrcToAny(route_src) => *src == route_src,
    //         MsgRoute::SrcToDst(route_src, route_dst) => *src == route_src && *dst == route_dst,
    //         MsgRoute::AnyToAny => true,
    //         MsgRoute::None => false,
    //         MsgRoute::SrcToDstSeveral(routes) => {
    //             for (route_src, route_dst) in routes {
    //                 if *src == route_src && *dst == route_dst {
    //                     return true;
    //                 }
    //             }
    //             false
    //         }
    //     }
    // }
}
