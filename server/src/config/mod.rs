use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env::current_dir, fs::File, io::Read};
use tracing::info;

#[derive(Debug, Serialize, Deserialize)]
pub struct App {
    pub name: String,
    pub version: String,
    pub host: String,
    pub port: u16,
    pub mysql: HashMap<String, MysqlItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MysqlItem {
    pub encrypt: bool,
    pub dsn: String,
}

pub fn init_config() -> Result<App> {
    let mut config_content = String::new();

    info!("running current dir: {}", current_dir().unwrap().display());

    let mut config_file = File::open("dev.yaml").context("open config file failed")?;

    config_file
        .read_to_string(&mut config_content)
        .context("read config content failed")?;

    let app: App =
        serde_yaml::from_str(&config_content).context("deserialize config content failed")?;

    info!("init config success: {:#?}", app);

    Ok(app)
}
