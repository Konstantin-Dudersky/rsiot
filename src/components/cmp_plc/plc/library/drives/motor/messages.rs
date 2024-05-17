use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct Status {
    pub man_act: bool, 
    pub aut_act: bool,
}
