use clap::Subcommand;
use std::{io, collections::HashMap, error::Error};
use dinstaller_lib::settings::Store as SettingsStore;
use dinstaller_lib::attributes::{Attributes, AttributeValue};
use crate::printers::{print, Format};

#[derive(Subcommand, Debug)]
pub enum ConfigCommands {
    /// Set one or many installation settings
    Set {
        /// key-value pairs (e.g., user.name="Jane Doe")
        values: Vec<String>,
    },
    /// Shows the value of one or many configuration settings
    Show
}

pub enum ConfigAction {
    Set(HashMap<String, String>),
    Show
}

pub fn run(subcommand: ConfigCommands, format: Option<Format>) -> Result<(), Box<dyn Error>> {
    match parse_config_command(subcommand) {
        ConfigAction::Set(changes) => {
            let store = SettingsStore::new()?;
            let mut model = store.load()?;
            for (key, value) in changes {
                // fixme: implement conversion from String to AttributeValue
                model.set_attribute(&key, AttributeValue(value))?;
            }
            store.store(&model)
        },
        ConfigAction::Show => {
            let store = SettingsStore::new()?;
            let model = store.load()?;
            print(model, io::stdout(), format)?;
            Ok(())
        }
    }
}

fn parse_config_command(subcommand: ConfigCommands) -> ConfigAction {
    match subcommand {
        ConfigCommands::Show => {
            ConfigAction::Show
        },
        ConfigCommands::Set { values } => {
            let changes: HashMap<String, String> = values.iter().filter_map(|s| {
                if let Some((key, value)) = s.split_once('=') {
                    Some((key.to_string(), value.to_string()))
                } else {
                    None
                }
            }).collect();
            ConfigAction::Set(changes)
        }
    }
}
