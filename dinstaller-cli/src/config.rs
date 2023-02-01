use clap::Subcommand;
use std::{collections::HashMap, error::Error, str::FromStr};
use dinstaller_lib::settings::{
    ItemsRepository, Key as SettingsKey, Store as SettingsStore
};

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
    Set(HashMap<SettingsKey, String>),
    Show(Vec<SettingsKey>)
}

pub fn run(subcommand: ConfigCommands) -> Result<(), Box<dyn Error>> {
    match parse_config_command(subcommand) {
        ConfigAction::Set(changes) => {
            let store = SettingsStore::new()?;
            let mut model = store.load()?;
            let settings_items = ItemsRepository::default_repository()?;

            for (key, value) in &changes {
                if let Some(item) = settings_items.find_by_key(key) {
                    (item.update_handler)(&mut model, value)
                }
            }
            store.store(&model)
        },
        _ => unimplemented!()
    }
}

fn parse_config_command(subcommand: ConfigCommands) -> ConfigAction {
    match subcommand {
        ConfigCommands::Show { keys } => {
            let keys = keys.iter().filter_map(|k| SettingsKey::from_str(&k).ok()).collect();
            ConfigAction::Show(keys)
        },
        ConfigCommands::Set { values } => {
            let changes: HashMap<SettingsKey, String> = values.iter().map(|s| {
                let (key, value) = s.split_once("=").unwrap();
                let key = SettingsKey::from_str(key).unwrap();
                (key, value.to_string())
            }).collect();
            ConfigAction::Set(changes)
        }
    }
}
