use clap::Parser;

mod commands;
mod config;
mod error;
mod printers;
mod profile;

use crate::error::CliError;
use async_std::task::{self, block_on};
use commands::Commands;
use config::run as run_config_cmd;
use dinstaller_lib::error::ServiceError;
use dinstaller_lib::manager::ManagerClient;
use indicatif::ProgressBar;
use printers::Format;
use profile::run as run_profile_cmd;
use std::error::Error;
use std::time::Duration;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Format output
    #[arg(value_enum, short, long, default_value_t = Format::Json)]
    pub format: Format,
}

async fn probe(manager: &ManagerClient<'_>) -> Result<(), Box<dyn Error>> {
    let another_manager = build_manager().await?;
    let probe = task::spawn(async move { another_manager.probe().await });
    show_progress(manager).await?;

    Ok(probe.await?)
}

async fn install(manager: &ManagerClient<'_>) -> Result<(), Box<dyn Error>> {
    if !manager.can_install().await? {
        // TODO: add some hints what is wrong or add dedicated command for it?
        eprintln!("There are issues with configuration. Cannot install.");
        return Err(Box::new(CliError::ValidationError));
    }
    let another_manager = build_manager().await?;
    let install = task::spawn(async move { another_manager.install().await });
    show_progress(manager).await?;

    Ok(install.await?)
}

async fn show_progress(client: &ManagerClient<'_>) -> Result<(), ServiceError> {
    // wait 1 second to give other task chance to start, so progress can display something
    task::sleep(Duration::from_secs(1)).await;
    let mut progress = client.progress().await?;
    eprintln!("Showing progress with max steps {:?}", progress.max_steps);
    let pb = ProgressBar::new(progress.max_steps.into());
    loop {
        if progress.finished {
            pb.finish();
            return Ok(());
        }
        pb.set_position(progress.current_step.into()); // TODO: display also title somewhere
        std::thread::sleep(std::time::Duration::from_secs(1));
        progress = client.progress().await?;
    }
}

async fn wait_for_services(manager: &ManagerClient<'_>) -> Result<(), Box<dyn Error>> {
    let services = manager.busy_services().await?;
    // TODO: having it optional
    if !services.is_empty() {
        eprintln!("There are busy services {services:?}. Waiting for them.");
        show_progress(manager).await?
    }
    Ok(())
}

async fn build_manager<'a>() -> Result<ManagerClient<'a>, Box<dyn Error>> {
    let conn = dinstaller_lib::connection().await?;
    Ok(ManagerClient::new(conn).await?)
}

async fn run_command(cli: Cli) -> Result<(), Box<dyn Error>> {
    match cli.command {
        Commands::Config(subcommand) => {
            let manager = build_manager().await?;
            block_on(wait_for_services(&manager))?;
            block_on(run_config_cmd(subcommand, cli.format))
        }
        Commands::Probe => {
            let manager = build_manager().await?;
            block_on(wait_for_services(&manager))?;
            block_on(probe(&manager))
        }
        Commands::Profile(subcommand) => Ok(run_profile_cmd(subcommand)?),
        Commands::Install => {
            let manager = build_manager().await?;
            block_on(wait_for_services(&manager))?;
            block_on(install(&manager))
        }
        _ => unimplemented!(),
    }
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    if let Err(error) = run_command(cli).await {
        eprintln!("{}", error);
        return Err(error);
    }
    Ok(())
}
