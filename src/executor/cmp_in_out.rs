use std::{cmp::max, fmt::Debug};

use tracing::{info, trace};
use uuid::Uuid;

use crate::message::{system_messages::*, *};

use super::{
    types::{CmpInput, CmpOutput, FnAuth},
    Cache, ComponentError,
};

/// Подключение компонента к внутренней шине сообщений исполнителя
#[derive(Debug)]
pub struct CmpInOut<TMsg, TService>
where
    TMsg: MsgDataBound,
    TService: ServiceBound,
{
    input: CmpInput<TMsg>,
    output: CmpOutput<TMsg>,
    /// Ссылка на кэш
    /// TODO - проверить, скорее всего можно сделать приватным
    pub cache: Cache<TMsg>,
    name: String,
    id: Uuid,
    auth_perm: AuthPermissions,
    fn_auth: FnAuth<TMsg>,

    /// Название текущего сервиса
    service: TService,
}

impl<TMsg, TService> CmpInOut<TMsg, TService>
where
    TMsg: MsgDataBound,
    TService: ServiceBound,
{
    /// Создание подключения к внутренней шине сообщений исполнителя
    pub fn new(
        input: CmpInput<TMsg>,
        output: CmpOutput<TMsg>,
        cache: Cache<TMsg>,
        name: &str,
        id: Uuid,
        auth_perm: AuthPermissions,
        fn_auth: FnAuth<TMsg>,
        service: TService,
    ) -> Self {
        info!("Start: {}, id: {}, auth_perm: {:?}", name, id, auth_perm);
        Self {
            input,
            output,
            cache,
            id,
            name: name.into(),
            auth_perm,
            fn_auth,
            service,
        }
    }

    /// Клонировать и присвоить новый идентификатор
    ///
    /// Необходимо вызывать в начале исполнения компонента, чтобы у каждого компонента был
    /// уникальный id
    pub fn clone_with_new_id(&self, name: &str, auth_perm: AuthPermissions) -> Self {
        let name = format!("{}::{}", self.name, name);
        let id = MsgTrace::generate_uuid();
        info!("Start: {}, id: {}, auth_perm: {:?}", name, id, auth_perm);
        Self {
            input: self.input.resubscribe(),
            output: self.output.clone(),
            cache: self.cache.clone(),
            name,
            id,
            auth_perm,
            fn_auth: self.fn_auth,
            service: self.service.clone(),
        }
    }

    /// Получение сообщений со входа
    pub async fn recv_input(&mut self) -> Result<Message<TMsg>, ComponentError> {
        loop {
            let msg = self
                .input
                .recv()
                .await
                .map_err(|e| ComponentError::CmpInput(e.to_string()))?;

            // Обновляем уровень авторизации при получении системного сообщения. Пропускаем
            // сообщение, если запрос на авторизацию не проходил через данный компонент
            if let MsgData::System(System::AuthResponseOk(value)) = &msg.data {
                if !value.trace_ids.contains(&self.id) {
                    continue;
                }
                self.auth_perm = max(self.auth_perm, value.perm);
            }
            if let MsgData::System(System::AuthResponseErr(value)) = &msg.data {
                if !value.trace_ids.contains(&self.id) {
                    continue;
                }
            }

            // Если данное сообщение было сгенерировано данным сервисом, пропускаем
            if msg.contains_trace_item(&self.id) {
                continue;
            }

            // Если нет авторизации, пропускаем
            let Some(mut msg) = (self.fn_auth)(msg, &self.auth_perm) else {
                continue;
            };

            msg.add_trace_item(&self.id, &self.name);
            return Ok(msg);
        }
    }

    /// Возвращает копию сообщений из кеша
    pub async fn recv_cache_all(&self) -> Vec<Message<TMsg>> {
        let lock = self.cache.read().await;
        lock.values()
            .cloned()
            .filter_map(|m| (self.fn_auth)(m, &self.auth_perm))
            .collect()
    }

    /// Возвращает сообщение из кеша по ключу
    pub async fn recv_cache_msg(&self, key: &str) -> Option<Message<TMsg>> {
        let cache = self.cache.read().await;
        cache.get(key).map(|m| m.to_owned())
    }

    /// Отправка сообщений на выход
    pub async fn send_output(&self, msg: Message<TMsg>) -> Result<(), ComponentError> {
        trace!("Start send to output: {msg:?}");
        // Если нет авторизации, пропускаем
        let Some(mut msg) = (self.fn_auth)(msg, &self.auth_perm) else {
            trace!("No authorization. Auth: {:?}", self.auth_perm);
            return Ok(());
        };

        msg.add_trace_item(&self.id, &self.name);
        self.output
            .send(msg)
            .await
            .map_err(|e| ComponentError::CmpOutput(e.to_string()))
    }

    /// Отправка исходящих сообщений, в синхронном окружении
    pub fn send_output_blocking(&self, msg: Message<TMsg>) -> Result<(), ComponentError> {
        trace!("Start send to output: {msg:?}");
        // Если нет авторизации, пропускаем
        let Some(mut msg) = (self.fn_auth)(msg, &self.auth_perm) else {
            trace!("No authorization. Auth: {:?}", self.auth_perm);
            return Ok(());
        };

        msg.add_trace_item(&self.id, &self.name);
        self.output
            .blocking_send(msg)
            .map_err(|e| ComponentError::CmpOutput(e.to_string()))
    }

    /// Возвращает максимальный размер очереди сообщений
    pub fn max_capacity(&self) -> usize {
        self.output.max_capacity()
    }
}

impl<TMsg, TService> Clone for CmpInOut<TMsg, TService>
where
    TMsg: MsgDataBound,
    TService: ServiceBound,
{
    fn clone(&self) -> Self {
        Self {
            input: self.input.resubscribe(),
            output: self.output.clone(),
            cache: self.cache.clone(),
            id: self.id,
            name: self.name.clone(),
            auth_perm: self.auth_perm,
            fn_auth: self.fn_auth,
            service: self.service.clone(),
        }
    }
}
