use clap::Subcommand;
use dinstaller_lib::profile::{download, ProfileEvaluator, ProfileValidator, ValidationResult};
use std::{error::Error, path::Path};

#[derive(Subcommand, Debug)]
pub enum ProfileCommands {
    /// Download the profile from a given location
    Download { url: String },

    /// Validate a profile using JSON Schema
    Validate { path: String },

    /// Evaluate a profile, injecting the hardware information from D-Bus
    Evaluate { path: String },
}

fn validate(path: String) -> Result<(), Box<dyn Error>> {
    let validator = ProfileValidator::default_schema()?;
    let path = Path::new(&path);
    let result = validator.validate_file(path)?;
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

fn evaluate(path: String) -> Result<(), Box<dyn Error>> {
    let evaluator = ProfileEvaluator {};
    evaluator.evaluate(Path::new(&path))
}

pub fn run(subcommand: ProfileCommands) -> Result<(), Box<dyn Error>> {
    match subcommand {
        ProfileCommands::Download { url } => download(&url),
        ProfileCommands::Validate { path } => validate(path),
        ProfileCommands::Evaluate { path } => evaluate(path),
    }
}
