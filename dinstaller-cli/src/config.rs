use crate::printers::{print, Format};
use clap::Subcommand;
use dinstaller_lib::attributes::{AttributeValue, Attributes};
use dinstaller_lib::Store as SettingsStore;
use std::{collections::HashMap, error::Error, io};

#[derive(Subcommand, Debug)]
pub enum ConfigCommands {
    /// Add an element to a collection
    Add { key: String, value: String },
    /// Set one or many installation settings
    Set {
        /// key-value pairs (e.g., user.name="Jane Doe")
        values: Vec<String>,
    },
    /// Shows the value of one or many configuration settings
    Show,
}

pub enum ConfigAction {
    Add(String, String),
    Set(HashMap<String, String>),
    Show,
}

pub fn run(subcommand: ConfigCommands, format: Option<Format>) -> Result<(), Box<dyn Error>> {
    let store = SettingsStore::new()?;
    let mut model = store.load()?;

    match parse_config_command(subcommand) {
        ConfigAction::Set(changes) => {
            for (key, value) in changes {
                model.set(&key, AttributeValue(value))?;
            }
            store.store(&model)
        }
        ConfigAction::Show => {
            print(model, io::stdout(), format)?;
            Ok(())
        }
        ConfigAction::Add(key, value) => {
            model.add(&key, AttributeValue(value))?;
            store.store(&model)
        }
    }
}

fn parse_config_command(subcommand: ConfigCommands) -> ConfigAction {
    match subcommand {
        ConfigCommands::Add { key, value } => ConfigAction::Add(key, value),
        ConfigCommands::Show => ConfigAction::Show,
        ConfigCommands::Set { values } => {
            let changes: HashMap<String, String> = values
                .iter()
                .filter_map(|s| {
                    if let Some((key, value)) = s.split_once('=') {
                        Some((key.to_string(), value.to_string()))
                    } else {
                        None
                    }
                })
                .collect();
            ConfigAction::Set(changes)
        }
    }
}
