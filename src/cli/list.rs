use std::collections::HashMap;

use colored::*;
use eso_addons::addons::Manager;
use eso_addons::config::Config;
use prettytable::{format, Table};

#[derive(Clap)]
pub struct ListCommand {}

impl ListCommand {
    pub fn run(
        &self,
        addon_manager: &Manager,
        config: &Config,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut table = Table::new();

        let mut addon_status: HashMap<String, Vec<String>> = HashMap::new();

        let format = format::FormatBuilder::new()
            .column_separator('|')
            .borders('|')
            .padding(1, 1)
            .separators(
                &[
                    format::LinePosition::Top,
                    format::LinePosition::Bottom,
                    format::LinePosition::Title,
                ],
                format::LineSeparator::new('-', '+', '+', '+'),
            )
            .build();
        table.set_format(format);

        table.set_titles(row!["Name".bold(), "Status".bold()]);

        let desired_addons = &config.addons;
        let installed_addons = addon_manager.get_addons()?;

        for addon in desired_addons {
            addon_status.insert(addon.name.clone(), vec![]);

            match installed_addons.iter().find(|a| a.name == addon.name) {
                Some(addon) => addon_status
                    .get_mut(&addon.name)
                    .map(|x| x.push("INSTALLED".green().to_string())),
                None => addon_status
                    .get_mut(&addon.name)
                    .map(|x| x.push("NOT INSTALLED".truecolor(255, 255, 0).to_string())),
            };
        }

        for addon in eso_addons::get_missing_dependencies(&installed_addons).into_iter() {
            if !addon_status.contains_key(&addon) {
                addon_status.insert(addon.clone(), vec![]);
            }

            addon_status
                .get_mut(&addon)
                .map(|x| x.push("MISSING".red().to_string()));
        }
        for addon in eso_addons::get_unused_dependencies(&installed_addons, desired_addons) {
            if !addon_status.contains_key(&addon) {
                addon_status.insert(addon.clone(), vec![]);
            }

            addon_status
                .get_mut(&addon)
                .map(|x| x.push("UNUSED".truecolor(100, 100, 100).to_string()));
        }

        for (k, v) in addon_status {
            let status = v.join(", ");
            table.add_row(row![k, status]);
        }

        table.printstd();

        Ok(())
    }
}
