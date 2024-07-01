use std::time::Duration;

use rsiot::{
    components::{cmp_derive, cmp_http_client, cmp_influxdb, cmp_logger},
    executor::{ComponentExecutor, ComponentExecutorConfig},
    message::{Message, MsgData, Timestamp},
};
use serde_json::from_str;
use tracing::Level;
use url::Url;

use messages::*;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    // cmp_derive ----------------------------------------------------------------------------------
    #[derive(Clone, Default, PartialEq)]
    struct WaterLevelStore {
        pressure1: Option<f64>,
        pressure2: Option<f64>,
    }

    let water_level = cmp_derive::DeriveItem {
        store: WaterLevelStore::default(),
        fn_input: |msg, store| match &msg.data {
            MsgData::Custom(data) => match data {
                Custom::Pressure1(content) => store.pressure1 = Some(content.pressure_Pa()),
                Custom::Pressure2(content) => store.pressure2 = Some(content.pressure_Pa()),
                _ => (),
            },
            _ => (),
        },
        fn_output: |store| {
            let msg_content = 0.10197 * (store.pressure1? - store.pressure2?);
            let msg = Message::new(MsgData::Custom(Custom::WaterLevel2(msg_content)));
            Some(vec![msg])
        },
    };

    let config_derive = cmp_derive::Config {
        derive_items: vec![Box::new(water_level)],
    };

    // cmp_influxdb --------------------------------------------------------------------------------
    let config_influxdb = cmp_influxdb::Config {
        host: "localhost".into(),
        port: 8086,
        org: "org".into(),
        bucket: "bucket".into(),
        token: "token".into(),
        fn_input: |msg: &Message<Custom>| {
            let value = match &msg.data {
                MsgData::Custom(Custom::Pressure1(data)) => {
                    cmp_influxdb::ValueType::f64(data.pressure_Pa())
                }
                MsgData::Custom(Custom::Pressure2(data)) => {
                    cmp_influxdb::ValueType::f64(data.pressure_Pa())
                }
                MsgData::Custom(Custom::WaterLevel2(data)) => cmp_influxdb::ValueType::f64(*data),
                _ => return None,
            };
            let ts = Timestamp::default();
            let line = cmp_influxdb::LineProtocolItem::new(&msg.key, value, &ts);
            Some(vec![line])
        },
    };

    // cmp_http_client -----------------------------------------------------------------------------
    let config_http_client = cmp_http_client::Config {
        connection_config: cmp_http_client::ConnectionConfig {
            base_url: Url::parse("http://192.168.71.1").unwrap(),
        },
        requests_input: vec![],
        requests_periodic: vec![cmp_http_client::RequestPeriodic {
            period: Duration::from_secs(2),
            http_param: cmp_http_client::HttpParam::Get {
                endpoint: "messages".to_string(),
            },
            on_success: |body| {
                let res = from_str::<Vec<Message<Custom>>>(body)?;
                Ok(res)
            },
            on_failure: Vec::new,
        }],
    };

    // cmp_logger ----------------------------------------------------------------------------------
    let _config_logger = cmp_logger::Config::<Custom> {
        level: Level::INFO,
        fn_input: |msg| {
            let text = msg.serialize()?;
            let text = format!("Header: {text}");
            Ok(Some(text))
        },
    };

    // executor ------------------------------------------------------------------------------------
    let config_executor = ComponentExecutorConfig {
        buffer_size: 100,
        executor_name: "level_measure_bmp180".into(),
        fn_auth: |msg, _| Some(msg),
    };

    ComponentExecutor::<Custom>::new(config_executor)
        .add_cmp(cmp_http_client::Cmp::new(config_http_client))
        .add_cmp(cmp_influxdb::Cmp::new(config_influxdb))
        .add_cmp(cmp_derive::Cmp::new(config_derive))
        // .add_cmp(cmp_logger::Cmp::new(config_logger))
        .wait_result()
        .await
        .unwrap();
}
