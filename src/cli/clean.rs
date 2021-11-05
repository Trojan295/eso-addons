use colored::*;
use eso_addons::{
    addons::{Addon, Manager},
    config::Config,
};
#[derive(Parser)]
pub struct CleanCommand {
    #[clap(long)]
    remove: Option<bool>,
}

impl CleanCommand {
    pub fn run(
        &mut self,
        config: &Config,
        addon_manager: &Manager,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let desired_addons = &config.addons;
        let installed_addons = addon_manager.get_addons()?;

        let unmanaged = eso_addons::get_unmanaged_addons(&desired_addons, installed_addons.iter());

        if unmanaged.len() > 0 {
            match self.remove {
                Some(true) => self.remove_addons(addon_manager, unmanaged.iter())?,
                Some(false) => self.show_addons_to_remove(unmanaged.iter()),
                None => {
                    self.show_addons_to_remove(unmanaged.iter());
                    if self.ask_for_remove_confirmation()? {
                        println!("");
                        self.remove_addons(addon_manager, unmanaged.iter())?;
                    }
                }
            }
        } else {
            println!("Nothing to clean");
        }
        Ok(())
    }

    fn ask_for_remove_confirmation(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let question = requestty::Question::confirm("remove")
            .message("Do you want to remove these addons?")
            .build();

        let answer = requestty::prompt_one(question)?;
        Ok(answer.as_bool().unwrap_or(false))
    }

    fn show_addons_to_remove<'a, I>(&self, addons: I)
    where
        I: Iterator<Item = &'a &'a Addon>,
    {
        println!("{} Addons to remove:", "🗑".red());

        for addon in addons {
            println!("- {}", addon.name)
        }

        println!("")
    }

    fn remove_addons<'a, I>(
        &self,
        addon_manager: &Manager,
        addons: I,
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        I: Iterator<Item = &'a &'a Addon>,
    {
        for addon in addons {
            addon_manager.delete_addon(addon)?;
            println!("{} {} removed!", "✓".green(), addon.name)
        }

        Ok(())
    }
}
