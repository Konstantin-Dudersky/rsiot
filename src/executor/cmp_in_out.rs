use std::{cmp::max, fmt::Debug};

use tracing::{error, info, trace, warn};
use uuid::Uuid;

use crate::message::{system_messages::*, *};

use super::{
    ComponentError, MsgBusInput, MsgBusOutput,
    types::{CmpInput, CmpOutput, FnAuth},
};

/// Подключение компонента к внутренней шине сообщений исполнителя
#[derive(Debug)]
pub struct CmpInOut<TMsg>
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

impl<TMsg> CmpInOut<TMsg>
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

    /// Клонировать и присвоить новый идентификатор
    ///
    /// Необходимо вызывать в начале исполнения компонента, чтобы у каждого компонента был
    /// уникальный id
    #[deprecated]
    pub fn clone_with_new_id(self, name: &str, auth_perm: AuthPermissions) -> Self {
        let id = Uuid::new_v4();
        info!("Start: {}, id: {}, auth_perm: {:?}", name, id, auth_perm);
        Self {
            input: self.input,
            output: self.output,
            name: name.into(),
            id,
            auth_perm,
            fn_auth: self.fn_auth,
        }
    }

    /// Компонент и получает, и отправляет сообщения
    #[deprecated]
    pub fn msgbus_input_output(self, name: &str) -> (MsgBusInput<TMsg>, MsgBusOutput<TMsg>) {
        let id = Uuid::new_v4();
        info!("Start: {}, id: {}", name, id);

        let input = MsgBusInput::new(self.input, name, id);
        let output = MsgBusOutput::new(self.output, id);
        (input, output)
    }

    /// Компонент только получает входящие сообщения
    #[deprecated]
    pub fn msgbus_input(self, name: &str) -> MsgBusInput<TMsg> {
        let id = Uuid::new_v4();
        info!("Start: {}, id: {}", name, id);
        MsgBusInput::new(self.input, name, id)
    }

    /// Компонент только отправляет исходящие сообщения
    #[deprecated]
    pub fn msgbus_output(self, name: &str) -> MsgBusOutput<TMsg> {
        let id = Uuid::new_v4();
        info!("Start: {}, id: {}", name, id);
        MsgBusOutput::new(self.output, id)
    }

    /// Получение сообщений со входа
    #[deprecated]
    pub async fn recv_input(&mut self) -> Result<Message<TMsg>, ComponentError> {
        loop {
            let msg = self.input.recv().await;

            let msg = match msg {
                Ok(v) => v,
                Err(err) => {
                    warn!(
                        "MsgBus.recv_input() of component {} input error: {}",
                        self.name, err
                    );
                    continue;
                }
            };

            // Обновляем уровень авторизации при получении системного сообщения. Пропускаем
            // сообщение, если запрос на авторизацию не проходил через данный компонент
            if let MsgData::System(System::AuthResponseOk(value)) = &msg.data {
                if !value.trace_ids.contains(&self.id) {
                    continue;
                }
                self.auth_perm = max(self.auth_perm, value.perm);
            }
            if let MsgData::System(System::AuthResponseErr(value)) = &msg.data
                && !value.trace_ids.contains(&self.id)
            {
                continue;
            }

            // Если данное сообщение было сгенерировано данным сервисом, пропускаем
            if msg.check_source(&self.id) {
                continue;
            }

            // Если нет авторизации, пропускаем
            let Some(msg) = (self.fn_auth)(msg, &self.auth_perm) else {
                continue;
            };

            return Ok(msg);
        }
    }

    /// Отправка сообщений на выход
    #[deprecated]
    pub async fn send_output(&self, msg: Message<TMsg>) -> Result<(), ComponentError> {
        trace!("Start send to output: {msg:?}");
        // Если нет авторизации, пропускаем
        let Some(mut msg) = (self.fn_auth)(msg, &self.auth_perm) else {
            trace!("No authorization. Auth: {:?}", self.auth_perm);
            return Ok(());
        };

        msg.set_cmp_source(&self.id);
        self.output
            .send(msg)
            .await
            .map_err(|e| ComponentError::CmpOutput(e.to_string()))
    }

    /// Отправка исходящих сообщений, в синхронном окружении
    #[deprecated]
    pub fn send_output_blocking(&self, msg: Message<TMsg>) -> Result<(), ComponentError> {
        trace!("Start send to output: {msg:?}");
        // Если нет авторизации, пропускаем
        let Some(mut msg) = (self.fn_auth)(msg, &self.auth_perm) else {
            trace!("No authorization. Auth: {:?}", self.auth_perm);
            return Ok(());
        };

        msg.set_cmp_source(&self.id);

        self.output
            .blocking_send(msg)
            .map_err(|e| ComponentError::CmpOutput(e.to_string()))
    }
}

impl<TMsg> Clone for CmpInOut<TMsg>
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
