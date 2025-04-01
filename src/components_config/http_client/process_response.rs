use tracing::warn;

use crate::{
    components_config::http_general::HttpDataBound,
    message::MsgDataBound,
    serde_utils::{SerdeAlg, SerdeAlgKind},
};

use super::{FnProcessResponseError, FnProcessResponseSuccess, MsgResponse};

pub fn process_response<TMsg, TServerToClient>(
    serde_alg: SerdeAlgKind,
    fn_process_response_success: FnProcessResponseSuccess<TMsg, TServerToClient>,
    fn_process_response_error: FnProcessResponseError<TMsg>,
    msg_response: &MsgResponse,
) -> Option<Vec<TMsg>>
where
    TMsg: 'static + MsgDataBound,
    TServerToClient: 'static + HttpDataBound,
{
    let msgs = match msg_response {
        // Пустое тело ответа - скорее всего тип TServerToClient = Unit
        MsgResponse::Success { body, .. } if body.is_empty() => return None,

        MsgResponse::Success { body, .. } => {
            let serde_alg = SerdeAlg::new(serde_alg);
            let body: Result<TServerToClient, _> = serde_alg.deserialize(body);
            let body = match body {
                Ok(v) => v,
                Err(e) => {
                    let err = format!("Deserialization error: {}", e);
                    warn!("{}", err);
                    return None;
                }
            };
            (fn_process_response_success)(&body)
        }

        MsgResponse::Error { error, .. } => {
            warn!("Error processing http response: {}", error);
            (fn_process_response_error)()
        }
    };

    Some(msgs)
}
