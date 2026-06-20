use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::{env, fs::File};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub service: ServiceConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServiceConfig {
    pub port: u16,
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        let res = match (
            File::open("app.yml"),
            File::open("./etc/config/app.yml"),
            env::var("CHAT_CONFIG"),
        ) {
            (Ok(read), _, _) => serde_yaml::from_reader(read),
            (_, Ok(read), _) => serde_yaml::from_reader(read),
            (_, _, Ok(path)) => serde_yaml::from_reader(File::open(path)?),
            _ => bail!("config file  not found"),
        };
        Ok(res?)
    }
}
