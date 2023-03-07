use clap::Subcommand;
use dinstaller_lib::profile;
use std::error::Error;
use std::path::Path;

#[derive(Subcommand, Debug)]
pub enum ProfileCommands {
    /// Download the profile from given location
    Download { url: String },

    /// Validate a given profile
    Validate,

    /// Evaluate the profile
    Evaluate,
}

pub async fn run(subcommand: ProfileCommands) -> Result<(), Box<dyn Error>> {
    match subcommand {
        ProfileCommands::Download { url } => profile::download(&url),
        ProfileCommands::Validate => unimplemented!(),
        ProfileCommands::Evaluate => unimplemented!(),
    }
}
