use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Status {
    pub man_act: bool,
    pub aut_act: bool,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum ManCommand {
    #[default]
    NoCommand,
    Start,
    Stop,
}
