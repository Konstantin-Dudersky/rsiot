#![allow(clippy::module_inception)]

use std::{fmt::Debug, time::Duration};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{MsgData, MsgDataBound, MsgTrace, TimeToLiveValue, Timestamp};

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
    /// Сервис, в котором было созданно данное сообщение.
    ///
    /// Устанавливается в исполнителе.
    service_origin: Option<String>,
}

impl<TCustom> Message<TCustom>
where
    TCustom: MsgDataBound,
{
    /// Создать новое сообщение
    pub fn new(data: MsgData<TCustom>) -> Self {
        let key = define_key(&data);
        let ttl = data.define_time_to_live();
        Self {
            data,
            key,
            ts: Default::default(),
            trace: MsgTrace::default(),
            ttl,
            service_origin: None,
        }
    }

    /// Создать новое сообщение типа `MsgData::Custom`
    pub fn new_custom(custom_data: TCustom) -> Self {
        let data = MsgData::Custom(custom_data);
        let key = define_key(&data);
        let ttl = data.define_time_to_live();
        Self {
            data,
            key,
            ts: Default::default(),
            trace: MsgTrace::default(),
            ttl,
            service_origin: None,
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
    pub fn add_trace_item(&mut self, id: &Uuid, name: &str) {
        self.trace.add_trace_item(*id, name.to_string())
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

    /// Возращает название сервиса, в котором было создано данное сообщение.
    /// Паникует, если название сервиса еще не установлено
    pub fn service_origin(&self) -> String {
        match &self.service_origin {
            Some(service_origin) => service_origin.clone(),
            None => panic!("service_origin not set"),
        }
    }

    /// Устанавливает название сервиса, в котором было создано данное сообщение.
    /// Если название уже установлено, то пропускаем
    pub fn set_service_origin(&mut self, service: &str) {
        match self.service_origin {
            Some(_) => (),
            None => self.service_origin = Some(service.to_string()),
        }
    }

    /// Разрешен ли марштур данного сообщения
    pub fn is_route_enabled(&self, src: TCustom::TService, dst: TCustom::TService) -> bool {
        let enabled_routes = match &self.data {
            MsgData::System(data) => return data.define_enabled_routes(),
            MsgData::Custom(data) => data.define_enabled_routes(),
        };
        for (src_enabled, dst_enabled) in enabled_routes {
            if let Some(src_enabled) = src_enabled {
                if src != src_enabled {
                    continue;
                }
            }
            if let Some(dst_enabled) = dst_enabled {
                if dst != dst_enabled {
                    continue;
                }
            }
            return true;
        }
        false
    }
}

/// Определить ключ сообщения по выводу Debug
fn define_key<TCustom>(data: &MsgData<TCustom>) -> String
where
    TCustom: MsgDataBound,
{
    let full_str = format!("{:?}", data);
    let key = full_str.split('(').collect::<Vec<&str>>();
    // Убираем последний элемент. Если тип unit (), нужно убрать два последних элемента
    let skip = if key[key.len() - 2].is_empty() { 2 } else { 1 };
    key[0..key.len() - skip].join("-")
}
