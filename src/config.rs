use super::errors::*;
use serde::ser::SerializeStruct;
use serde_derive::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Deserialize, Debug, Clone)]
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
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    #[serde(rename = "addonDir")]
    pub addon_dir: PathBuf,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub addons: Vec<AddonEntry>,
}

impl serde::Serialize for AddonEntry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("AddonEntry", 0)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("url", &self.url)?;
        state.serialize_field("dependency", &self.dependency)?;

        state.end()
    }
}

pub fn parse_config(path: &Path) -> Result<Config, Box<dyn Error>> {
    if !path.exists() {
        create_initial_config(path)?;
    }

    let config_data = fs::read_to_string(path)
        .chain_err(&format!("while reading config file at {}", path.display()))?;
    let config: Config = toml::from_str(&config_data)?;
    Ok(config)
}

pub fn save_config(path: &Path, cfg: &Config) -> Result<(), Box<dyn Error>> {
    let config_str = toml::to_string(cfg)?;
    fs::write(path, config_str)
        .chain_err(&format!("while writing config file at {}", path.display()))?;
    Ok(())
}

fn create_initial_config(path: &Path) -> Result<(), Box<dyn Error>> {
    let config = get_initial_config();
    save_config(path, &config)?;
    Ok(())
}

#[cfg(target_os = "windows")]
fn get_initial_config() -> Config {
    let home_dir = dirs::home_dir().unwrap();
    let addon_dir = home_dir.join("Documents/Elder Scrolls Online/live/AddOns");

    Config {
        addon_dir: addon_dir.display().to_string(),
        addons: vec![],
    }
}

#[cfg(target_os = "linux")]
fn get_initial_config() -> Config {
    let home_dir = dirs::home_dir().unwrap();
    let addon_dir =
        home_dir.join("drive_c/users/user/My Documents/Elder Scrolls Online/live/AddOns");

    Config {
        addon_dir: addon_dir,
        addons: vec![],
    }
}

#[cfg(target_os = "macos")]
fn get_initial_config() -> Config {
    Config {
        addon_dir: String::from(""),
        addons: vec![],
    }
}
