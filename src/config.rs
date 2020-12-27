use super::errors::*;
use serde_derive::Deserialize;
use std::error::Error;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct AddonEntry {
    pub name: String,
    pub url: Option<String>,
    #[serde(default = "default_dependency")]
    pub dependency: bool,
}

fn default_dependency() -> bool {
    false
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(rename = "addonDir")]
    pub addon_dir: String,
    pub addons: Vec<AddonEntry>,
}

pub fn parse_config(path: &str) -> Result<Config, Box<dyn Error>> {
    let config_data =
        fs::read_to_string(path).chain_err(&format!("while reading config file at {}", path))?;
    let config: Config = toml::from_str(&config_data)?;
    Ok(config)
}
