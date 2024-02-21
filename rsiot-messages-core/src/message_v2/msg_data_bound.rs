use std::fmt::Debug;

use serde::{de::DeserializeOwned, Serialize};

pub trait MsgDataBound: Clone + Debug + DeserializeOwned + Send + Serialize + Sync {}
