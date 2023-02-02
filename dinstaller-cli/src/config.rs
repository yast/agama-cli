use clap::Subcommand;
use std::{collections::HashMap, error::Error};
use dinstaller_lib::settings::Store as SettingsStore;
use dinstaller_lib::attributes::{Attributes, AttributeValue};

#[derive(Subcommand, Debug)]
pub enum ConfigCommands {
    /// Set one or many installation settings
    Set {
        /// key-value pairs (e.g., user.name="Jane Doe")
        values: Vec<String>,
    },
    /// Shows the value of one or many configuration settings
    Show {
        /// Keys to show
        keys: Vec<String>,
    },
}

pub enum ConfigAction {
    Set(HashMap<String, String>),
    Show(Vec<String>)
}

pub fn run(subcommand: ConfigCommands) -> Result<(), Box<dyn Error>> {
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
        _ => unimplemented!()
    }
}

fn parse_config_command(subcommand: ConfigCommands) -> ConfigAction {
    match subcommand {
        ConfigCommands::Show { keys } => {
            ConfigAction::Show(keys)
        },
        ConfigCommands::Set { values } => {
            let changes: HashMap<String, String> = values.iter().map(|s| {
                let (key, value) = s.split_once("=").unwrap();
                // let key = SettingsKey::from_str(key).unwrap();
                (key.to_string(), value.to_string())
            }).collect();
            ConfigAction::Set(changes)
        }
    }
}
