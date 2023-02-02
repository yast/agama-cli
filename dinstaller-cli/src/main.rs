use clap::Parser;
use std::error;

use dinstaller_cli::commands::{Commands, ConfigCommands};
use dinstaller_lib::{connection,software, storage, users, manager};
use dinstaller_cli::printers::{print, Format};

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
        "users" => print(users::first_user()?, stdout, format),
        "storage.candidate_devices" => print(storage::candidate_devices()?, stdout, format),
        "storage.available_devices" => print(storage::available_devices()?, stdout, format),
        "products" => print(software::products(), stdout, format),
        _ => {
            println!("unknown key");
            Ok(())
        }
    }
}

fn show_config(keys: Vec<String>) {
    unimplemented!("Show config for {:?}", &keys);
}

fn set_config(values: Vec<String>) {
    unimplemented!("Set config values {:?}", &values);
}

fn probe() {
    let client = manager::ManagerClient::new(connection().unwrap()).unwrap();
    client.probe().unwrap()
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Info { keys } => info(keys, cli.format).unwrap(),
        Commands::Config(subcommand) => match subcommand {
            ConfigCommands::Show { keys } => show_config(keys),
            ConfigCommands::Set { values } => set_config(values),
        },
        Commands::Probe => probe()
    }
}
