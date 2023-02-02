use clap::Subcommand;
use crate::config::ConfigCommands;

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
}
