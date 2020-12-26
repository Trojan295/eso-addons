use clap::Clap;
use dirs;
use eso_addons::addons;
use eso_addons::config;

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
}

#[derive(Clap)]
struct List {}

#[derive(Clap)]
struct Update {}

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

            Ok(())
        }
        SubCommand::Update(_) => {
            let desired_addons = config.addons;

            for addon in desired_addons.iter() {
                let addon = addon_manager.download_addon(&addon.url)?;
                println!("{}, {:?} installed", addon.name, addon.depends_on);
            }

            Ok(())
        }
    }
}
