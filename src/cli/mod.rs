use clap::Clap;
use dirs;
use eso_addons::addons;
use eso_addons::config;
use std::path::PathBuf;

mod add;
mod clean;
mod list;
mod remove;
mod update;

#[derive(Clap)]
#[clap(
    version = crate_version!(),
    author = crate_authors!(),
    about = "CLI tool for managing addons for The Elder Scrolls Online"
)]
struct Opts {
    #[clap(short, long, about = "Path to TOML config file")]
    config: Option<String>,
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    #[clap(about = "Lists status of installed addons")]
    List(list::ListCommand),
    #[clap(about = "Installs and updates addons")]
    Update(update::UpdateCommand),
    #[clap(about = "Uninstall not managed and unused addons")]
    Clean(clean::CleanCommand),
    #[clap(about = "Adds a new addon to the configuration")]
    Add(add::AddCommand),
    #[clap(about = "Uninstall and remove addon from config file")]
    Remove(remove::RemoveCommand),
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let opts: Opts = Opts::parse();

    let home_dir = dirs::home_dir().unwrap();

    let default_config_filepath = home_dir.join(".eso-addons.toml");
    let config_filepath = opts
        .config
        .map(|x| PathBuf::from(&x))
        .unwrap_or(default_config_filepath);

    let mut config = config::parse_config(&config_filepath)?;

    let addon_manager = addons::Manager::new(&config.addon_dir);

    match opts.subcmd {
        SubCommand::List(list) => list.run(&addon_manager, &config),
        SubCommand::Update(update) => update.run(&config, &addon_manager),
        SubCommand::Clean(clean) => clean.run(&config, &addon_manager),
        SubCommand::Add(add) => add.run(&mut config, &config_filepath, &addon_manager),
        SubCommand::Remove(remove) => remove.run(&mut config, &config_filepath, &addon_manager),
    }
}
