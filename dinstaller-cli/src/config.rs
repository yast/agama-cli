use crate::printers::{print, Format};
use clap::Subcommand;
use dinstaller_lib::install_settings::{InstallSettings, Scope};
use dinstaller_lib::settings::{SettingObject, SettingValue, Settings};
use dinstaller_lib::Store as SettingsStore;
use std::str::FromStr;
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

    match parse_config_command(subcommand) {
        ConfigAction::Set(changes) => {
            let scopes = changes.keys().filter_map(|k| key_to_scope(k)).collect();
            let mut model = store.load(Some(scopes))?;
            for (key, value) in changes {
                model.set(&key, SettingValue(value))?;
            }
            store.store(&model)
        }
        ConfigAction::Show => {
            let model = store.load(None)?;
            print(model, io::stdout(), format)?;
            Ok(())
        }
        ConfigAction::Add(key, values) => {
            let scope = key_to_scope(&key).unwrap();
            let mut model = store.load(Some(vec![scope]))?;
            model.add(&key, SettingObject::from(values))?;
            store.store(&model)
        }
        ConfigAction::Load(path) => {
            let contents = std::fs::read_to_string(path)?;
            let result: InstallSettings = serde_json::from_str(&contents).unwrap();
            let scopes = result.defined_scopes();
            let mut model = store.load(Some(scopes))?;
            model.merge(&result);
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

fn key_to_scope(key: &str) -> Option<Scope> {
    if let Some((name, _)) = key.split_once('.') {
        return Scope::from_str(name).ok();
    }
    None
}
