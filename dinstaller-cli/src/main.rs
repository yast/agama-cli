mod cli;
mod printers;
mod commands;

use clap::Parser;
use std::error;
use std::str::FromStr;

use cli::{Commands, ConfigCommands};
use commands::config::{ConfigAction, ConfigKey, ConfigAssignment};
use printers::{print, Format};
use dinstaller_lib::{software, storage, users};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Format output
    #[arg(value_enum, short, long)]
    pub format: Option<Format>
}

/// Displays information about a given configuration parameter
///
/// This function does not handle the `keys` argument properly yet.
fn info(keys: Vec<String>, format: Option<Format>) -> Result<(), Box<dyn error::Error>> {
    let products = "products".to_string();
    let key = keys.get(0)
        .unwrap_or(&products);

    let stdout = std::io::stdout();
    match key.as_str() {
        "users" => print(users::users(), stdout, format),
        "storage.candidate_devices" => print(storage::candidate_devices()?, stdout, format),
        "storage.available_devices" => print(storage::available_devices()?, stdout, format),
        "products" => print(software::products(), stdout, format),
        _ => {
            println!("unknown key");
            Ok(())
        }
    }
}

/// Extracts the config action from the command line
fn build_config_action(subcommand: ConfigCommands) -> ConfigAction {
    // fixme: instead of filtering, we should report errors
    match subcommand {
        ConfigCommands::Show { keys } => {
            let keys = keys.iter().filter_map(|k| ConfigKey::from_str(&k).ok()).collect();
            ConfigAction::Show(keys)
        },
        ConfigCommands::Set { values } => {
            let values = values.iter().filter_map(|k| ConfigAssignment::from_str(&k).ok()).collect();
            ConfigAction::Set(values)
        }
    }
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Info { keys } => info(keys, cli.format).unwrap(),
        Commands::Config(subcommand) => {
            let action = build_config_action(subcommand);
            dbg!(action);
        }
    }
}
