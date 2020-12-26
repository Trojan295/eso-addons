use serde_derive::Deserialize;
use std::error::Error;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct AddonEntry {
  pub name: String,
  #[serde(default)]
  pub url: String,
  pub dependency: bool,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct Config {
  #[serde(rename = "addonDir")]
  pub addon_dir: String,
  pub addons: Vec<AddonEntry>,
}

pub fn parse_config(path: &str) -> Result<Config, Box<dyn Error>> {
  let config_data = fs::read_to_string(path)?;
  let config: Config = toml::from_str(&config_data)?;
  Ok(config)
}
