use crate::message::MsgDataBound;

use super::{GetEndpointsCollection, PutEndpointsCollection};

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
    get_endpoints: &GetEndpointsCollection<TMsg>,
    put_endpoints: &PutEndpointsCollection<TMsg>,
) -> String
where
    TMsg: MsgDataBound,
{
    let get = get_endpoints.info();
    let put = put_endpoints.info();

    TEMPLATE_INFO.replace("#GET#", &get).replace("#PUT#", &put)
}
