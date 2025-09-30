use std::fmt::Debug;

use tracing::{error, info};
use uuid::Uuid;

use crate::message::{AuthPermissions, MsgDataBound};

use super::{
    MsgBusInput, MsgBusOutput,
    types::{CmpInput, CmpOutput, FnAuth},
};

/// Подключение компонента к внутренней шине сообщений исполнителя
#[derive(Debug)]
pub struct MsgBusLinker<TMsg>
where
    TMsg: MsgDataBound,
{
    input: CmpInput<TMsg>,
    output: CmpOutput<TMsg>,
    name: String,
    id: Uuid,
    auth_perm: AuthPermissions,
    fn_auth: FnAuth<TMsg>,
}

impl<TMsg> MsgBusLinker<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Создание подключения к внутренней шине сообщений исполнителя
    pub fn new(
        input: CmpInput<TMsg>,
        output: CmpOutput<TMsg>,
        auth_perm: AuthPermissions,
        fn_auth: FnAuth<TMsg>,
    ) -> Self {
        Self {
            input,
            output,
            id: Uuid::default(),
            name: "".to_string(),
            auth_perm,
            fn_auth,
        }
    }

    /// Инициализация шины сообщений с новым идентификатором и именем
    pub fn init(mut self, name: &str) -> Self {
        let id = Uuid::new_v4();
        self.id = id;
        self.name = name.into();
        info!("Start: {}, id: {}", name, id);
        self
    }

    /// Канал входящих сообщений
    pub fn input(&self) -> MsgBusInput<TMsg> {
        if self.name.is_empty() {
            error!("Component name is empty");
            panic!("Component name is empty");
        }
        MsgBusInput::new(self.input.resubscribe(), self.name.clone(), self.id)
    }

    /// Канал исходящих сообщений
    pub fn output(&self) -> MsgBusOutput<TMsg> {
        if self.name.is_empty() {
            error!("Component name is empty");
            panic!("Component name is empty");
        }
        MsgBusOutput::new(self.output.clone(), self.id)
    }

    /// Каналы входящих сообщений и исходящих сообщений
    pub fn input_output(&self) -> (MsgBusInput<TMsg>, MsgBusOutput<TMsg>) {
        (self.input(), self.output())
    }

    /// Возвращает максимальный размер очереди сообщений
    pub fn max_capacity(&self) -> usize {
        self.output.max_capacity()
    }

    /// Закрыть подключение
    pub fn close(self) {}
}

impl<TMsg> Clone for MsgBusLinker<TMsg>
where
    TMsg: MsgDataBound,
{
    fn clone(&self) -> Self {
        Self {
            input: self.input.resubscribe(),
            output: self.output.clone(),
            id: self.id,
            name: self.name.clone(),
            auth_perm: self.auth_perm,
            fn_auth: self.fn_auth,
        }
    }
}
