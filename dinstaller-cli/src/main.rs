use clap::Parser;

mod commands;
mod config;
mod printers;
mod profile;

use async_std::task::{self, block_on};
use commands::Commands;
use dinstaller_lib::manager::ManagerClient;
use indicatif::ProgressBar;
use printers::Format;
use std::time::Duration;

use config::run as run_config_cmd;
use profile::run as run_profile_cmd;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Format output
    #[arg(value_enum, short, long)]
    pub format: Option<Format>,
}

async fn probe(manager: &ManagerClient<'_>) {
    let probe = task::spawn(async {
        // use new manager here
        let another_manager = ManagerClient::new(dinstaller_lib::connection())
            .await
            .unwrap();
        another_manager.probe().await.unwrap()
    });
    block_on(show_progress(&manager));

    probe.await
}

async fn install(manager: &ManagerClient<'_>) {
    if !manager.can_install().await.unwrap() {
        // TODO: add some hints what is wrong or add dedicated command for it?
        eprintln!("There are issues with configuration. Cannot install.");
        return;
    }
    let install = task::spawn(async {
        // use new manager here
        let another_manager = ManagerClient::new(dinstaller_lib::connection())
            .await
            .unwrap();
        another_manager.install().await.unwrap()
    });
    block_on(show_progress(manager));

    install.await
}

async fn show_progress(client: &ManagerClient<'_>) {
    // wait 1 second to give other task chance to start, so progress can display something
    task::sleep(Duration::from_secs(1)).await;
    let mut progress = client.progress().await.unwrap();
    eprintln!("Showing progress with max steps {:?}", progress.max_steps);
    let pb = ProgressBar::new(progress.max_steps.into());
    loop {
        if progress.finished {
            pb.finish();
            return;
        }
        pb.set_position(progress.current_step.into()); // TODO: display also title somewhere
        std::thread::sleep(std::time::Duration::from_secs(1));
        progress = client.progress().await.unwrap();
    }
}

async fn wait_for_services(manager: &ManagerClient<'_>) {
    let services = manager.busy_services().await.unwrap();
    // TODO: having it optional
    if !services.is_empty() {
        eprintln!("There are busy services {services:?}. Waiting for them.");
        show_progress(manager).await
    }
}

fn main() {
    block_on( async {
        let manager = ManagerClient::new(
            dinstaller_lib::connection(),
        )
        .await.unwrap();
        // get all attributes to proxy, so later we can rely on signals when dbus service will be blocked
        manager.progress().await.unwrap().max_steps;
        wait_for_services(&manager).await;
        let cli = Cli::parse();
        match cli.command {
            Commands::Config(subcommand) => run_config_cmd(subcommand, cli.format).await.unwrap(),
            Commands::Probe => probe(&manager).await,
            Commands::Profile(subcommand) => run_profile_cmd(subcommand).unwrap(),  
            Commands::Install => install(&manager).await,
            _ => unimplemented!(),
        }
    } )
}
