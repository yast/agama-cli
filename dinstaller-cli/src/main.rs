use clap::Parser;

mod commands;
mod config;
mod printers;

use commands::Commands;
use config::run as run_config_cmd;
use printers::Format;
use indicatif::ProgressBar;
use dinstaller_lib::manager;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Format output
    #[arg(value_enum, short, long)]
    pub format: Option<Format>,
}

fn probe() {
    let client = manager::ManagerClient::new(dinstaller_lib::connection().unwrap()).unwrap();
    client.probe().unwrap();
    let mut progress = client.progress().unwrap();
    let pb = ProgressBar::new(progress.max_steps.into());
    loop {
        if progress.finished {
            pb.finish();
            return;
        }
        pb.set_position(progress.current_step.into()); // TODO: display also title somewhere
        std::thread::sleep(std::time::Duration::from_secs(1));
        progress = client.progress().unwrap();
    }
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Config(subcommand) => run_config_cmd(subcommand, cli.format).unwrap(),
        Commands::Probe => probe(),
        _ => unimplemented!(),
    }
}
