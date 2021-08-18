use std::path::Path;

use eso_addons::{
    addons::Manager,
    config::{self, Config},
};

#[derive(Clap)]
pub struct RemoveCommand {
    name: String,
}

impl RemoveCommand {
    pub fn run(
        &self,
        config: &mut Config,
        config_filepath: &Path,
        addon_manager: &Manager,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let idx = config
            .addons
            .iter()
            .position(|entry| entry.name == self.name)
            .ok_or(simple_error!("failed to find addon {}", &self.name))?;

        let entry = config.addons.remove(idx);

        let addon = addon_manager.get_addon(&entry.name)?;
        if let Some(addon) = addon {
            addon_manager.delete_addon(&addon)?;
        }

        config::save_config(config_filepath, config)?;

        println!("Uninstalled and removed addon {}!", &entry.name);

        Ok(())
    }
}
