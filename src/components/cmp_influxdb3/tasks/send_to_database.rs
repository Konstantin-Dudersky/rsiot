use reqwest::{Client, StatusCode};
use tokio::sync::mpsc;
use tracing::{trace, warn};

use crate::components::cmp_influxdb3::LineProtocolItem;

use super::{send_to_database_message::SendToDatabaseMessage, Error, Result};

pub struct SendToDatabase {
    pub input: mpsc::Receiver<SendToDatabaseMessage>,
    pub url: String,
    pub database: String,
}

impl SendToDatabase {
    pub async fn spawn(mut self) -> Result<()> {
        let mut cache = vec![];

        while let Some(msg) = self.input.recv().await {
            match msg {
                SendToDatabaseMessage::LineProtocolItem(item) => cache.push(item),
                SendToDatabaseMessage::SendByTimer => {
                    handle_request(&self.url, &self.database, &cache).await?;
                    cache.clear();
                }
            }
        }
        Err(Error::TaskSendToDatabase)
    }
}

async fn handle_request(
    url: &str,
    database: &str,
    datapoints: &[LineProtocolItem],
) -> super::Result<()> {
    trace!("New request to InfluxDB");

    let lines = datapoints
        .iter()
        .filter_map(|lpi| {
            let s = lpi.to_string();
            match s {
                Ok(v) => Some(v),
                Err(e) => {
                    warn!("{}", e);
                    None
                }
            }
        })
        .collect::<Vec<String>>()
        .join("\n");

    let client = Client::new();
    let response = client
        .post(url)
        // .header("Authorization", format!("Token {}", config.token))
        .header("Accept", "application/json")
        .header("Content-Type", "text/plain; charset=utf-8")
        .query(&[("db", database), ("precision", "nanosecond")])
        .body(lines)
        .send()
        .await;

    let response = match response {
        Ok(v) => v,
        Err(e) => {
            warn!("Error sending data to InfluxDB: {:?}", e);
            return Ok(());
        }
    };

    let status = response.status();
    if status == StatusCode::NO_CONTENT {
        return Ok(());
    }
    warn!("{status}");
    let text = response.text().await?;
    Err(Error::RequestParameters {
        status,
        message: text,
    })
}
