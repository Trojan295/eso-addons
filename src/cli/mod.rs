use clap::Clap;
use dirs;
use eso_addons::addons;
use eso_addons::config;
use eso_addons::errors::*;

#[derive(Clap)]
#[clap(version = "0.1.0", author = "Damian C. <trojan295@gmail.com>")]
struct Opts {
    #[clap(short, long)]
    config: Option<String>,
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    List(List),
    Update(Update),
    Clean(Clean),
}

#[derive(Clap)]
struct List {}

#[derive(Clap)]
struct Update {}

#[derive(Clap)]
struct Clean {
    #[clap(long)]
    remove: bool,
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let opts: Opts = Opts::parse();

    let home_dir = dirs::home_dir().unwrap();
    let default_config_path = format!("{}/.eso-addons.toml", home_dir.display());

    let config = config::parse_config(&opts.config.unwrap_or(default_config_path))?;

    let addon_manager = addons::Manager::new(&config.addon_dir);

    match opts.subcmd {
        SubCommand::List(_) => {
            let desired_addons = config.addons;
            let installed_addons = addon_manager.get_addons()?;

            for addon in desired_addons {
                match installed_addons.iter().find(|a| a.name == addon.name) {
                    Some(_) => println!("{} is installed", addon.name),
                    None => println!("{} is not installed", addon.name),
                }
            }

            let missing_addons: Vec<String> =
                eso_addons::get_missing_dependencies(&installed_addons).collect();

            if missing_addons.len() > 0 {
                println!("\nThere are missing addons:");

                for missing in eso_addons::get_missing_dependencies(&installed_addons) {
                    println!("- {}", missing);
                }
            }

            Ok(())
        }
        SubCommand::Update(_) => {
            let desired_addons = config.addons;

            for addon in desired_addons.iter() {
                let installed = if let Some(ref url) = addon.url {
                    let installed = addon_manager
                        .download_addon(&url)
                        .chain_err(&format!("while downloading {}", addon.name))?;
                    Some(installed)
                } else {
                    addon_manager.get_addon(&addon.name)?
                };

                if let Some(installed) = installed {
                    if installed.name == addon.name {
                        println!("{} installed", addon.name);
                    } else {
                        println!(
                        "WARNING: {} is called {} is config file. Verify the addon name in the config file.",
                        installed.name, addon.name
                    );
                    }
                } else {
                    println!(
                        "WARNING: {} is set to be manually installed, but not present",
                        addon.name
                    )
                }
            }

            let installed_addons = addon_manager.get_addons()?;
            let missing_addons: Vec<String> =
                eso_addons::get_missing_dependencies(&installed_addons).collect();

            if missing_addons.len() > 0 {
                println!("\nThere are missing addons:");

                for missing in eso_addons::get_missing_dependencies(&installed_addons) {
                    println!("- {}", missing);
                }
            }

            Ok(())
        }
        SubCommand::Clean(clean) => {
            let desired_addons = config.addons;
            let installed_addons = addon_manager.get_addons()?;

            let unmanaged = eso_addons::get_unmanaged_addons(&desired_addons, &installed_addons);

            if unmanaged.len() > 0 {
                if clean.remove {
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
}
