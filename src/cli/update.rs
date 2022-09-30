use colored::*;
use eso_addons::{addons::Manager, config::Config};

use super::errors::*;

#[derive(Parser)]
pub struct UpdateCommand {}

impl UpdateCommand {
    pub fn run(&self, config: &Config, addon_manager: &Manager) -> Result<()> {
        let desired_addons = &config.addons;

        for addon in desired_addons.iter() {
            let installed = if let Some(ref url) = addon.url {
                let installed = match addon_manager.download_addon(&url) {
                    Ok(installed) => installed,
                    Err(e) => {
                        println!("{} Failed {}!", "☒".red(), addon.name);
                        println!("{}", e.to_string());
                        continue;
                    }
                };
                Some(installed)
            } else {
                addon_manager.get_addon(&addon.name)?
            };

            if let Some(installed) = installed {
                if installed.name == addon.name {
                    println!("{} Updated {}!", "✔".green(), addon.name);
                } else {
                    println!(
                        // TODO: change the name in the config automatically
                        "⚠ Installed {}, but is called {} is config file. Verify the addon name in the config file.",
                        installed.name, addon.name
                    );
                }
            } else {
                println!(
                    "⚠ {} is set to be manually installed, but not present",
                    addon.name
                )
            }
        }

        let installed_addons_list = addon_manager.get_addons()?;
        let missing_addons: Vec<String> =
            eso_addons::get_missing_dependencies(&installed_addons_list.addons).collect();

        if missing_addons.len() > 0 {
            println!(
                "\n{} There are missing dependencies! Please install the following addons to resolve the dependencies:",
                "⚠".red()
            );

            for missing in eso_addons::get_missing_dependencies(&installed_addons_list.addons) {
                println!("- {}", missing);
            }
        }

        let unused_addons =
            eso_addons::get_unused_dependencies(&installed_addons_list.addons, desired_addons);

        if unused_addons.len() > 0 {
            println!("\nThere are unused dependencies:");

            for unused in unused_addons {
                println!("- {}", unused);
            }
        }

        Ok(())
    }
}
