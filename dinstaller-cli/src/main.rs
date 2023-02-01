use clap::Parser;

mod commands;
mod config;
mod printers;

use commands::Commands;
use config::run as run_config_cmd;
use printers::Format;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Format output
    #[arg(value_enum, short, long)]
    pub format: Option<Format>
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Config(subcommand) => run_config_cmd(subcommand).unwrap(),
        _ => unimplemented!()
    }
}
