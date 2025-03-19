use std::collections::HashMap;

use crate::message::{Message, MsgDataBound};

use super::{GetEndpoint, GetEndpointsHashMap, PutEndpoint, PutEndpointsHashMap};

/// Обработка GET запроса
pub fn handler_get<TMsg, TError>(
    path: &str,
    get_endpoints: &HashMap<String, Box<dyn GetEndpoint<TMsg>>>,
    error_unknown_path: fn(String) -> TError,
    error_serde_json: fn(serde_json::Error) -> TError,
) -> Result<String, TError> {
    get_endpoints
        .get(path)
        .ok_or_else(|| error_unknown_path(path.to_string()))?
        .get_json_data()
        .map_err(error_serde_json)
}

/// Обработка PUT запроса
pub fn handler_put<TMsg, TError>(
    path: &str,
    body: &str,
    put_endpoints: &HashMap<String, Box<dyn PutEndpoint<TMsg>>>,
    error_unknown_path: fn(String) -> TError,
    error_serde_json: fn(serde_json::Error) -> TError,
) -> Result<Option<Message<TMsg>>, TError> {
    put_endpoints
        .get(path)
        .ok_or_else(|| error_unknown_path(path.to_string()))?
        .fn_output(body)
        .map_err(error_serde_json)
}

const TEMPLATE_INFO: &str = r#"
<p>
    GET:
    <ul>
        #GET#
    </ul>
</p>
<p>
    PUT:
    <ul>
        #PUT#
    </ul>
</p>
"#;

/// Вывод перечня доступных точек
pub fn handler_info<TMsg>(
    get_endpoints: &GetEndpointsHashMap<TMsg>,
    put_endpoints: &PutEndpointsHashMap<TMsg>,
) -> String
where
    TMsg: MsgDataBound,
{
    let get = get_endpoints
        .keys()
        .map(|k| format!("<li>{k}</li>"))
        .collect::<Vec<String>>()
        .join("\n");

    let put = put_endpoints
        .keys()
        .map(|k| format!("<li>{k}</li>"))
        .collect::<Vec<String>>()
        .join("\n");

    TEMPLATE_INFO.replace("#GET#", &get).replace("#PUT#", &put)
}
