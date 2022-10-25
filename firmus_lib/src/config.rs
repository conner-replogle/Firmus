use std::{path::PathBuf, time::Duration};

use serde::{Deserialize, Serialize};

//program config
#[derive(Deserialize,Serialize,Default)]
pub struct Config{
    pub name: String,
    pub directory_path: PathBuf,
    pub restart_on_fail: bool,
    pub run_command: String,
    pub metrics: Option<MetricSettings>,
    pub git_settings: Option<GitSettings>
}
#[derive(Deserialize,Serialize,Default)]
pub struct GitSettings{
    pub polling_rate: Duration,
    pub url: String,
    pub username: String,
    pub auth_key: String,
    pub update_command: String
}
#[derive(Deserialize,Serialize,Default)]
pub struct MetricSettings{
    pub url: String

}