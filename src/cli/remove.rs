use std::path::Path;

use colored::*;

use eso_addons::{
    addons::Manager,
    config::{self, Config},
};

use super::{Error, Result};

#[derive(Parser)]
pub struct RemoveCommand {
    name: Option<String>,
}

impl RemoveCommand {
    pub fn run(
        &self,
        config: &mut Config,
        config_filepath: &Path,
        addon_manager: &Manager,
    ) -> Result<()> {
        let addon_name = match &self.name {
            Some(name) => name.to_owned(),
            None => self.ask_for_addon_name(addon_manager)?,
        };

        let idx = config
            .addons
            .iter()
            .position(|entry| entry.name == addon_name)
            .ok_or(Error::Other(Box::new(super::errors::Error::AddonNotFound(
                addon_name,
            ))))?;

        let entry = config.addons.remove(idx);

        let addon = addon_manager.get_addon(&entry.name)?;
        if let Some(addon) = addon {
            addon_manager.delete_addon(&addon)?;
        }

        config::save_config(config_filepath, config)?;

        println!("{} Uninstalled {}!", "✔".green(), &entry.name);

        Ok(())
    }

    fn ask_for_addon_name(&self, addon_manager: &Manager) -> Result<String> {
        let addons: Vec<String> = addon_manager
            .get_addons()?
            .addons
            .iter()
            .map(|addon| addon.name.clone())
            .collect();

        if addons.is_empty() {
            return Err(Error::NoAddonsInstalled);
        }

        let question = requestty::Question::select("addon_name")
            .message("Select addon to remove")
            .choices(addons)
            .build();

        let answer = requestty::prompt_one(question).map_err(|err| Error::Other(Box::new(err)))?;

        Ok(answer.as_list_item().map(|item| item.text.clone()).unwrap())
    }
}
