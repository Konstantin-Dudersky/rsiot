//! Компонент авторизации пользователей
//!
//! TODO - переделать, поскольку удалил MsgTrace

mod component;
mod config;
mod error;
mod fn_process;
mod token_payload;

#[cfg(test)]
mod test;

pub use component::Cmp;
pub use config::{Config, ConfigStore, ConfigStoreLocalItem};
pub use error::Error;

type Result<TMsg> = std::result::Result<TMsg, Error>;

// Старый код проверки авторизации в шине сообщений
//
// /// Получение сообщений со входа
// #[deprecated]
// pub async fn recv_input(&mut self) -> Result<Message<TMsg>, ComponentError> {
//     loop {
//         let msg = self.input.recv().await;

//         let msg = match msg {
//             Ok(v) => v,
//             Err(err) => {
//                 warn!(
//                     "MsgBus.recv_input() of component {} input error: {}",
//                     self.name, err
//                 );
//                 continue;
//             }
//         };

//         // Обновляем уровень авторизации при получении системного сообщения. Пропускаем
//         // сообщение, если запрос на авторизацию не проходил через данный компонент
//         if let MsgData::System(System::AuthResponseOk(value)) = &msg.data {
//             if !value.trace_ids.contains(&self.id) {
//                 continue;
//             }
//             self.auth_perm = max(self.auth_perm, value.perm);
//         }
//         if let MsgData::System(System::AuthResponseErr(value)) = &msg.data
//             && !value.trace_ids.contains(&self.id)
//         {
//             continue;
//         }

//         // Если данное сообщение было сгенерировано данным сервисом, пропускаем
//         if msg.check_source(&self.id) {
//             continue;
//         }

//         // Если нет авторизации, пропускаем
//         let Some(msg) = (self.fn_auth)(msg, &self.auth_perm) else {
//             continue;
//         };

//         return Ok(msg);
//     }
// }

// /// Отправка сообщений на выход
// #[deprecated]
// pub async fn send_output(&self, msg: Message<TMsg>) -> Result<(), ComponentError> {
//     trace!("Start send to output: {msg:?}");
//     // Если нет авторизации, пропускаем
//     let Some(mut msg) = (self.fn_auth)(msg, &self.auth_perm) else {
//         trace!("No authorization. Auth: {:?}", self.auth_perm);
//         return Ok(());
//     };

//     msg.set_cmp_source(&self.id);
//     self.output
//         .send(msg)
//         .await
//         .map_err(|e| ComponentError::CmpOutput(e.to_string()))
// }

// /// Отправка исходящих сообщений, в синхронном окружении
// #[deprecated]
// pub fn send_output_blocking(&self, msg: Message<TMsg>) -> Result<(), ComponentError> {
//     trace!("Start send to output: {msg:?}");
//     // Если нет авторизации, пропускаем
//     let Some(mut msg) = (self.fn_auth)(msg, &self.auth_perm) else {
//         trace!("No authorization. Auth: {:?}", self.auth_perm);
//         return Ok(());
//     };

//     msg.set_cmp_source(&self.id);

//     self.output
//         .blocking_send(msg)
//         .map_err(|e| ComponentError::CmpOutput(e.to_string()))
// }
