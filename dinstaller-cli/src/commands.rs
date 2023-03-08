use crate::config::ConfigCommands;
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Change or show installation settings
    #[command(subcommand)]
    Config(ConfigCommands),

    /// Display information about installation settings (e.g., possible values)
    Info {
        /// Configuration keys (e.g., software.products)
        keys: Vec<String>,
    },
    /// Start probing
    Probe,
    // Start Installation
    Install
}
