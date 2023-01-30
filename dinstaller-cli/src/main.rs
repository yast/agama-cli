use clap::Parser;
mod commands;
use commands::{Commands, ConfigCommands};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    pub command: commands::Commands,
}

fn info(keys: Vec<String>) {
    unimplemented!("Display information for {:?}", &keys);
}

fn show_config(keys: Vec<String>) {
    unimplemented!("Show config for {:?}", &keys);
}

fn set_config(values: Vec<String>) {
    unimplemented!("Set config values {:?}", &values);
}

fn main() {
    let cli = Cli::parse();
    dbg!(&cli.command);
    match cli.command {
        Commands::Info { keys } => info(keys),
        Commands::Config(subcommand) => match subcommand {
            ConfigCommands::Show { keys } => show_config(keys),
            ConfigCommands::Set { values } => set_config(values),
        },
    }
}
