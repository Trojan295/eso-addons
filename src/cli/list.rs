use eso_addons::addons::Manager;
use eso_addons::config::Config;

#[derive(Clap)]
pub struct ListCommand {}

impl ListCommand {
    pub fn run(
        &self,
        addon_manager: &Manager,
        config: &Config,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let desired_addons = &config.addons;
        let installed_addons = addon_manager.get_addons()?;

        for addon in desired_addons {
            match installed_addons.iter().find(|a| a.name == addon.name) {
                Some(_) => println!("{} is installed", addon.name),
                None => println!("WARNING: {} is not installed", addon.name),
            }
        }

        let missing_addons: Vec<String> =
            eso_addons::get_missing_dependencies(&installed_addons).collect();

        if missing_addons.len() > 0 {
            println!("\nThere are missing addons:");

            for missing in missing_addons {
                println!("- {}", missing);
            }
        }

        let unused_addons = eso_addons::get_unused_dependencies(&installed_addons, desired_addons);

        if unused_addons.len() > 0 {
            println!("\nThere are unused dependencies:");

            for unused in unused_addons {
                println!("- {}", unused);
            }
        }

        Ok(())
    }
}
