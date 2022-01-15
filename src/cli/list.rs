use std::collections::BTreeMap;

use super::{Error, Result};
use colored::*;
use eso_addons::addons::Manager;
use eso_addons::config::Config;
use prettytable::{format, Table};

#[derive(Parser)]
pub struct ListCommand {}

impl ListCommand {
    pub fn run(&self, addon_manager: &Manager, config: &Config) -> Result<()> {
        let mut table = Table::new();

        let mut addon_status: BTreeMap<String, Vec<String>> = BTreeMap::new();

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
        let installed_addons_list = addon_manager
            .get_addons()
            .map_err(|err| Error::AppError(err))?;

        for addon in desired_addons {
            addon_status.insert(addon.name.clone(), vec![]);

            match installed_addons_list
                .addons
                .iter()
                .find(|a| a.name == addon.name)
            {
                Some(addon) => addon_status
                    .get_mut(&addon.name)
                    .map(|x| x.push("INSTALLED".green().to_string())),
                None => addon_status
                    .get_mut(&addon.name)
                    .map(|x| x.push("NOT INSTALLED".truecolor(200, 200, 0).to_string())),
            };
        }

        for addon in eso_addons::get_missing_dependencies(&installed_addons_list.addons).into_iter()
        {
            if !addon_status.contains_key(&addon) {
                addon_status.insert(addon.clone(), vec![]);
            }

            addon_status
                .get_mut(&addon)
                .map(|x| x.push("MISSING".red().to_string()));
        }
        for addon in
            eso_addons::get_unused_dependencies(&installed_addons_list.addons, desired_addons)
        {
            if !addon_status.contains_key(&addon) {
                addon_status.insert(addon.clone(), vec![]);
            }

            addon_status
                .get_mut(&addon)
                .map(|x| x.push("UNUSED".truecolor(130, 130, 130).to_string()));
        }

        for (k, v) in addon_status {
            let status = v.join(", ");
            table.add_row(row![k, status]);
        }

        table.printstd();

        for err in installed_addons_list.errors {
            let msg = format!("WARNING: {}", err);
            println!("{}", msg.yellow());
        }

        Ok(())
    }
}
