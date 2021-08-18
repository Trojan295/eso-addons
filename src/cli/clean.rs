use eso_addons::{addons::Manager, config::Config};

#[derive(Clap)]
pub struct CleanCommand {
    #[clap(long)]
    remove: bool,
}

impl CleanCommand {
    pub fn run(
        &self,
        config: &Config,
        addon_manager: &Manager,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let desired_addons = &config.addons;
        let installed_addons = addon_manager.get_addons()?;

        let unmanaged = eso_addons::get_unmanaged_addons(&desired_addons, &installed_addons);

        if unmanaged.len() > 0 {
            if self.remove {
                println!("Removing addons:");

                for addon in unmanaged.iter() {
                    addon_manager.delete_addon(addon)?;
                    println!("- {}", addon.name)
                }
            } else {
                println!("Addons to remove:");

                for addon in unmanaged.iter() {
                    println!("- {}", addon.name)
                }
            }
        } else {
            println!("Nothing to clean");
        }
        Ok(())
    }
}
