use super::super::model::Row;

pub enum SendToDatabaseMessage {
    Rows(Vec<Row>),
    SendByTimer,
}
