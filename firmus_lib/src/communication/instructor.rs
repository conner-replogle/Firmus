use serde::{Deserialize, Serialize};

use crate::config::Config;


#[derive(Deserialize,Serialize)]
pub enum Command{
    Start(Config),
    Stop(String),//PID
    List,
}