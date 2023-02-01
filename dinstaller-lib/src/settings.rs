use crate::users::{UsersClient,FirstUser};
use crate::storage::StorageClient;
use std::{str::FromStr, error::Error};

#[derive(Debug, Default)]
pub struct Settings {
    pub users: Option<UsersSettings>,
    pub storage: Option<StorageSettings>
}

#[derive(Debug)]
pub struct UsersSettings {
    pub first_user: Option<FirstUser>
}

#[derive(Debug)]
pub struct StorageSettings {

}

/// Settings storage
///
/// It is responsible for loading and storing the settings in the D-Bus service.
pub struct Store<'a> {
    storage_client: StorageClient<'a>,
    users_client: UsersClient<'a>,
}

impl<'a> Store<'a> {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(
            Self {
                storage_client: StorageClient::new(super::connection()?)?,
                users_client: UsersClient::new(super::connection()?)?
            }
        )
    }

    /// Loads the installation settings from the D-Bus service
    pub fn load(&self) -> Result<Settings, Box<dyn Error>> {
        let mut settings = Settings::default();
        settings.users = Some(UsersSettings {
            first_user: Some(self.users_client.first_user()?)
        });
        Ok(settings)
    }

    /// Stores the given installation settings in the D-Bus service
    pub fn store(&self, settings: &Settings) -> Result<(), Box<dyn Error>> {
        if let Some(users_settings) = &settings.users {
            if let Some(first_user) = &users_settings.first_user {
                self.users_client.set_first_user(&first_user)?;
            }
        }
        Ok(())
    }
}

type UpdateFn = fn(&mut Settings, value: &str);

/// Represents a configuration item that can be handled by the CLI
///
/// It contains a key, a description and a function to update the settings model.
pub struct Item {
    pub key: Key,
    pub description: String,
    pub update_handler: UpdateFn
}

impl Item {
    pub fn new(key: Key, description: String, update_handler: UpdateFn) -> Self {
        Self { key, description, update_handler }
    }
}

/// Represents a key (the name) of a settings item
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Key(pub String, pub String);

impl FromStr for Key {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((ns, id)) = s.split_once(".") {
            return Ok(Self(ns.to_string(), id.to_string()))
        }
        Err(format!("not a valid configuration key: {}", s).to_string())
    }
}

/// Repository containing the known configuration items
///
/// It offers a mechanism to store and search for a given configuration item using its key.
pub struct ItemsRepository {
    config_items: Vec<Item>
}

impl ItemsRepository {
    pub fn new() -> Self {
        Self { config_items: vec![] }
    }

    pub fn add(&mut self, config_item: Item) {
        self.config_items.push(config_item)
    }

    pub fn find_by_key(&self, key: &Key) -> Option<&Item> {
        self.config_items.iter().find(|c| &c.key == key)
    }

    pub fn default_repository() -> Result<Self, Box<dyn Error>> {
        let mut repository = ItemsRepository::new();
        repository.add(
            Item::new("users.full_name".parse()?, "First user full name".to_string(), |s, value| {
                // FIXME: We can simplify this code quite a lot by extending the Settings class.
                if let Some(users) = &mut s.users {
                    if let Some(first_user) = &mut users.first_user {
                        first_user.full_name = value.to_string()
                    }
                }
            })
        );
        repository.add(
            Item::new("storage.lvm".parse()?, "Whether to enable LVM".to_string(), |_s, value| {
                println!("Setting LVM to {}", value);
            })
        );
        Ok(repository)
    }
}
