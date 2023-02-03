use crate::users::{UsersClient, FirstUser};
use std::{str::FromStr, error::Error, default::Default};
use crate::attributes::{Attributes, AttributeValue};
use dinstaller_derive::DInstallerAttributes;

#[derive(Debug, Default)]
pub struct Settings {
    pub user: UserSettings,
}

#[derive(Debug, Default, DInstallerAttributes)]
pub struct UserSettings {
    pub full_name: String,
    pub user_name: String,
    pub password: String,
    pub autologin: bool,
}

#[derive(Debug, DInstallerAttributes)]
pub struct StorageSettings {
    lvm: bool,
    encryption_password: String
}

impl Attributes for Settings {
    fn set_attribute(&mut self, attr: &str, value: AttributeValue) -> Result<(), &'static str> {
        if let Some((ns, id)) = attr.split_once(".") {
            match ns {
                "user" => {
                    self.user.set_attribute(id, value)?
                },
                _ => return Err("unknown attribute")
            }
        }
        Ok(())
    }
}

/// Settings storage
///
/// It is responsible for loading and storing the settings in the D-Bus service.
pub struct Store<'a> {
    users_client: UsersClient<'a>,
}

impl<'a> Store<'a> {
    pub fn new() -> Result<Self, zbus::Error> {
        Ok(
            Self {
                users_client: UsersClient::new(super::connection()?)?
            }
        )
    }

    /// Loads the installation settings from the D-Bus service
    pub fn load(&self) -> Result<Settings, Box<dyn Error>> {
        let first_user = self.users_client.first_user()?;
        let settings = Settings {
            user: UserSettings {
                user_name: first_user.user_name,
                autologin: first_user.autologin,
                full_name: first_user.full_name,
                password:  first_user.password
            }
        };
        Ok(settings)
    }

    /// Stores the given installation settings in the D-Bus service
    pub fn store(&self, settings: &Settings) -> Result<(), Box<dyn Error>> {
        dbg!("Storing the following settings", settings);
        // fixme: improve
        let first_user = FirstUser {
            user_name: settings.user.user_name.clone(),
            full_name: settings.user.full_name.clone(),
            autologin: settings.user.autologin.clone(),
            password: settings.user.password.clone(),
            ..Default::default()
        };
        self.users_client.set_first_user(&first_user)?;
        Ok(())
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