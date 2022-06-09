use clap::Parser;
use eso_addons::{
    addons,
    addons::Manager,
    config::{self, AddonEntry, Config},
    htmlparser,
};
use std::path::Path;

use super::{Error, Result};

#[derive(Parser)]
pub struct AddCommand {
    addon_url: Option<String>,
    #[clap(
        short,
        long,
        help = "Indicate, if the addon is only a dependency for another addon"
    )]
    #[clap(short)]
    dependency: bool,
}

impl AddCommand {
    pub fn run(
        &mut self,
        cfg: &mut Config,
        config_filepath: &Path,
        addon_manager: &Manager,
    ) -> Result<()> {
        let mut entry = self.get_entry()?;

        if cfg.addons.iter().find(|el| el.url == entry.url).is_some() {
            println!("Addon {} is already installed", &entry.name);
            return Ok(());
        }

        let installed = addon_manager.download_addon(&entry.url.clone().unwrap())?;

        if entry.name != installed.name {
            entry.name = installed.name;
        }

        cfg.addons.push(entry.clone());

        config::save_config(config_filepath, &cfg)?;

        println!("🎊 Installed {}!", &entry.name);

        Ok(())
    }

    pub fn get_entry(&mut self) -> Result<AddonEntry> {
        if self.addon_url.is_none() {
            self.ask_for_fields()?;
        }

        let addon_url = self
            .addon_url
            .clone()
            .ok_or(Error::Other("missing addon URL".into()))?;
        let dependency = self.dependency;

        let addon_name = htmlparser::get_document(&addon_url)
            .map(htmlparser::get_addon_name)?
            .ok_or(Error::Other("failed to get addon name".into()))?;

        let download_url = addons::get_download_url(&addon_url);

        Ok(AddonEntry {
            name: addon_name,
            url: download_url,
            dependency: dependency,
        })
    }

    fn ask_for_fields(&mut self) -> Result<()> {
        let questions = vec![
            requestty::Question::input("addon_url")
                .message("URL of the addon on esoui.com")
                .build(),
            requestty::Question::confirm("dependency")
                .message("Is addon only a dependency?")
                .default(false)
                .build(),
        ];

        let answers = requestty::prompt(questions).map_err(|err| Error::Other(Box::new(err)))?;

        if let Some(addon_url) = answers.get("addon_url") {
            self.addon_url = addon_url.as_string().map(|x| x.to_owned());
        };

        if let Some(dependency) = answers.get("dependency") {
            self.dependency = dependency.as_bool().unwrap_or(false);
        };

        Ok(())
    }
}
