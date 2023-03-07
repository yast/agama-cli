use clap::Subcommand;
use dinstaller_lib::profile::{download, ProfileValidator, ValidationResult};
use std::{error::Error, path::Path};

#[derive(Subcommand, Debug)]
pub enum ProfileCommands {
    /// Download the profile from given location
    Download { url: String },

    /// Validate a given profile
    Validate { path: String },

    /// Evaluate the profile
    Evaluate,
}

pub async fn run(subcommand: ProfileCommands) -> Result<(), Box<dyn Error>> {
    match subcommand {
        ProfileCommands::Download { url } => download(&url),
        ProfileCommands::Validate { path } => validate(path),
        ProfileCommands::Evaluate => unimplemented!(),
    }
}

fn validate(path: String) -> Result<(), Box<dyn Error>> {
    let validator = ProfileValidator::default_schema()?;
    let path = Path::new(&path);
    let result = validator.validate(path)?;
    match result {
        ValidationResult::Valid => {
            println!("The profile is valid")
        }
        ValidationResult::NotValid(errors) => {
            println!("The profile is not valid. Please, check the following errors:\n");
            for error in errors {
                println!("* {error}")
            }
        }
    }
    Ok(())
}
