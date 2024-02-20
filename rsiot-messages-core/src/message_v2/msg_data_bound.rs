use std::fmt::Debug;

use serde::{de::DeserializeOwned, Serialize};

pub trait MsgContentBound: Clone + Debug + DeserializeOwned + Send + Serialize + Sync {}
