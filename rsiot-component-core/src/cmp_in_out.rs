use std::{cmp::max, fmt::Debug};

use tracing::info;
use uuid::Uuid;

use rsiot_messages_core::{system_messages::*, *};

use crate::{
    types::{CmpInput, CmpOutput, FnAuth},
    ComponentError,
};

#[derive(Debug)]
pub struct CmpInOut<TMsg> {
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
    pub fn new(
        input: CmpInput<TMsg>,
        output: CmpOutput<TMsg>,
        name: &str,
        auth_perm: AuthPermissions,
        fn_auth: FnAuth<TMsg>,
    ) -> Self {
        let id = MsgTrace::generate_uuid();
        info!("Start: {}, id: {}, auth_perm: {:?}", name, id, auth_perm);
        Self {
            input,
            output,
            id,
            name: name.into(),
            auth_perm,
            fn_auth,
        }
    }

    pub fn clone_with_new_id(&self, name: &str, auth_perm: AuthPermissions) -> Self {
        let name = format!("{}::{}", self.name, name);
        let id = MsgTrace::generate_uuid();
        info!("Start: {}, id: {}, auth_perm: {:?}", name, id, auth_perm);
        Self {
            name,
            id,
            auth_perm,
            input: self.input.resubscribe(),
            output: self.output.clone(),
            fn_auth: self.fn_auth.clone(),
        }
    }

    /// Получение сообщений со входа
    pub async fn recv_input(&mut self) -> Result<Option<Message<TMsg>>, ComponentError> {
        let msg = self
            .input
            .recv()
            .await
            .map_err(|e| ComponentError::CmpInput(e.to_string()))?;

        // Обновляем уровень авторизации при получении системного сообщения. Пропускаем сообщение,
        // если запрос на авторизацию не проходил через данный компонент
        if let MsgData::System(System::AuthResponseOk(value)) = &msg.data {
            if !value.trace_ids.contains(&self.id) {
                return Ok(None);
            }
            self.auth_perm = max(self.auth_perm, value.perm);
        }

        // Если данное сообщение было сгенерировано данным сервисом, пропускаем
        if msg.contains_trace_item(&self.id) {
            return Ok(None);
        }

        // Если нет авторизации, пропускаем
        let Some(msg) = (self.fn_auth)(msg, &self.auth_perm) else {
            return Ok(None);
        };

        Ok(Some(msg))
    }

    /// Отправка сообщений на выход
    pub async fn send_output(&self, msg: Message<TMsg>) -> Result<(), ComponentError> {
        // Если нет авторизации, пропускаем
        let Some(mut msg) = (self.fn_auth)(msg, &self.auth_perm) else {
            return Ok(());
        };

        msg.add_trace_item(&self.id, &self.name);
        self.output
            .send(msg)
            .await
            .map_err(|e| ComponentError::CmpOutput(e.to_string()))
    }
}

impl<TMsg> Clone for CmpInOut<TMsg>
where
    TMsg: Clone,
{
    fn clone(&self) -> Self {
        Self {
            input: self.input.resubscribe(),
            output: self.output.clone(),
            id: self.id,
            name: self.name.clone(),
            auth_perm: self.auth_perm.clone(),
            fn_auth: self.fn_auth.clone(),
        }
    }
}
