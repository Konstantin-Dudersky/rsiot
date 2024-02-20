use std::{fmt::Debug, sync::Arc};

use axum::extract;
use serde::Serialize;

use crate::{error::Error, shared_state::SharedState};

/// Маршрут для ввода сообщений
pub async fn replace<TMsg>(
    extract::State(shared_state): extract::State<Arc<SharedState<TMsg>>>,
    body: String,
) -> Result<(), Error>
where
    TMsg: Clone + Debug + Serialize,
{
    let msg = (shared_state.config.fn_output)(&body).map_err(Error::FnInput)?;
    let msg = match msg {
        Some(val) => val,
        None => return Ok(()),
    };
    shared_state
        .output
        .send(msg)
        .await
        .map_err(Error::CmpOutput)?;
    Ok(())
}
