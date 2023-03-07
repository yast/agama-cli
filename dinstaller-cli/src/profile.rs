use clap::Subcommand;
use std::error::Error;

#[derive(Subcommand, Debug)]
pub enum ProfileCommands {
    /// Download the profile from given location
    Download,

    /// Validate a given profile
    Validate,

    /// Evaluate the profile
    Evaluate,
}

pub async fn run(subcommand: ProfileCommands) -> Result<(), Box<dyn Error>> {
    match subcommand {
        ProfileCommands::Download => unimplemented!(),
        ProfileCommands::Validate => unimplemented!(),
        ProfileCommands::Evaluate => unimplemented!(),
    }
}
