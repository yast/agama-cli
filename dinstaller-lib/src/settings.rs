use crate::users::{UsersClient, FirstUser};
use crate::storage::StorageClient;
use std::{str::FromStr, error::Error, default::Default};
use crate::attributes::{Attributes, AttributeValue};

#[derive(Debug, Default)]
pub struct Settings {
    pub users: UsersSettings,
}

#[derive(Debug, Default)]
pub struct UsersSettings {
    pub full_name: String,
    pub user_name: String,
    pub password: String,
    pub autologin: bool,
}

#[derive(Debug)]
pub struct StorageSettings {
    lvm: bool,
    encryption_password: String
}

impl Attributes for Settings {
    fn set_attribute(&mut self, attr: &str, value: AttributeValue) -> Result<(), &'static str> {
        if let Some((ns, id)) = attr.split_once(".") {
            match ns {
                "users" => {
                    self.users.set_attribute(id, value)?
                },
                _ => return Err("unknown attribute")
            }
        }
        Ok(())
    }
}

impl Attributes for UsersSettings {
    fn set_attribute(&mut self, attr: &str, value: AttributeValue) -> Result<(), &'static str> {
        match attr {
            "full_name" => self.full_name = value.try_into()?,
            "user_name" => self.user_name = value.try_into()?,
            "password" => self.password = value.try_into()?,
            "autologin" => self.autologin = value.try_into()?,
            _ => return Err("unknown attribute")
        }
        Ok(())
    }
}

impl Attributes for StorageSettings {
    fn set_attribute(&mut self, attr: &str, value: AttributeValue) -> Result<(), &'static str> {
        match attr {
            "lvm" => self.lvm = value.try_into()?,
            "encryption_password" => self.encryption_password = value.try_into()?,
            _ => return Err("unknown attribute")
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
    pub fn new() -> Result<Self, Box<dyn Error>> {
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
            users: UsersSettings {
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
        let first_user = FirstUser {
            user_name: settings.users.user_name.clone(),
            full_name: settings.users.full_name.clone(),
            autologin: settings.users.autologin.clone(),
            password: settings.users.password.clone(),
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
