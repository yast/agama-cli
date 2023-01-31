mod cli;
mod printers;
mod actions;

use clap::Parser;
use std::error;
use std::collections::HashMap;
use std::str::FromStr;

use cli::{Commands, ConfigCommands};
use actions::{ConfigAction, ConfigKey, StorageActionsRunner};
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
        "storage.proposal" => print(storage::proposal()?, stdout, format),
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
            let changes: HashMap<ConfigKey, String> = values.iter().map(|s| {
                let (key, value) = s.split_once("=").unwrap();
                let key = ConfigKey::from_str(key).unwrap();
                (key, value.to_string())
            }).collect();
            ConfigAction::Set(changes)
        }
    }
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Info { keys } => info(keys, cli.format).unwrap(),
        Commands::Config(subcommand) => {
            // fixme: move to a better place
            let action = build_config_action(subcommand);
            let runner = StorageActionsRunner::new().unwrap();
            runner.run(action).unwrap();
        }
    }
}
