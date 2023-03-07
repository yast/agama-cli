use crate::printers::{print, Format};
use clap::Subcommand;
use dinstaller_lib::install_settings::InstallSettings;
use dinstaller_lib::settings::{SettingObject, SettingValue, Settings};
use dinstaller_lib::Store as SettingsStore;
use std::{collections::HashMap, error::Error, io};

#[derive(Subcommand, Debug)]
pub enum ConfigCommands {
    /// Add an element to a collection
    Add { key: String, values: Vec<String> },
    /// Set one or many installation settings
    Set {
        /// key-value pairs (e.g., user.name="Jane Doe")
        values: Vec<String>,
    },
    /// Shows the value of one or many configuration settings
    Show,
    /// Loads the configuration from a JSON file
    Load { path: String },
}

pub enum ConfigAction {
    Add(String, HashMap<String, String>),
    Set(HashMap<String, String>),
    Show,
    Load(String),
}

pub fn run(subcommand: ConfigCommands, format: Option<Format>) -> Result<(), Box<dyn Error>> {
    let store = SettingsStore::new()?;
    let mut model = store.load()?;

    match parse_config_command(subcommand) {
        ConfigAction::Set(changes) => {
            for (key, value) in changes {
                model.set(&key, SettingValue(value))?;
            }
            store.store(&model)
        }
        ConfigAction::Show => {
            print(model, io::stdout(), format)?;
            Ok(())
        }
        ConfigAction::Add(key, values) => {
            model.add(&key, SettingObject::from(values))?;
            store.store(&model)
        }
        ConfigAction::Load(path) => {
            let contents = std::fs::read_to_string(path)?;
            let result: InstallSettings = serde_json::from_str(&contents).unwrap();
            model.merge(result);
            store.store(&model)
        }
    }
}

fn parse_config_command(subcommand: ConfigCommands) -> ConfigAction {
    match subcommand {
        ConfigCommands::Add { key, values } => ConfigAction::Add(key, parse_keys_values(values)),
        ConfigCommands::Show => ConfigAction::Show,
        ConfigCommands::Set { values } => ConfigAction::Set(parse_keys_values(values)),
        ConfigCommands::Load { path } => ConfigAction::Load(path),
    }
}

fn parse_keys_values(keys_values: Vec<String>) -> HashMap<String, String> {
    keys_values
        .iter()
        .filter_map(|s| {
            if let Some((key, value)) = s.split_once('=') {
                Some((key.to_string(), value.to_string()))
            } else {
                None
            }
        })
        .collect()
}
