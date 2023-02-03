use crate::attributes::{AttributeValue, Attributes};
use crate::users::{FirstUser, UsersClient};
use crate::software::SoftwareClient;
use dinstaller_derive::DInstallerAttributes;
use serde::Serialize;
use std::{default::Default, error::Error, str::FromStr};

#[derive(Debug, Default, Serialize)]
pub struct Settings {
    pub user: UserSettings,
    pub software: SoftwareSettings,
}

impl Attributes for Settings {
    fn set_attribute(&mut self, attr: &str, value: AttributeValue) -> Result<(), &'static str> {
        if let Some((ns, id)) = attr.split_once('.') {
            match ns {
                "software" => self.software.set_attribute(id, value)?,
                "user" => self.user.set_attribute(id, value)?,
                _ => return Err("unknown attribute"),
            }
        }
        Ok(())
    }
}

#[derive(Debug, Default, DInstallerAttributes, Serialize)]
pub struct UserSettings {
    pub full_name: String,
    pub user_name: String,
    pub password: String,
    pub autologin: bool,
}

#[derive(Debug, DInstallerAttributes, Serialize)]
pub struct StorageSettings {
    lvm: bool,
    encryption_password: String,
}

#[derive(Debug, Default, DInstallerAttributes, Serialize)]
pub struct SoftwareSettings {
    product: String
}

/// Settings storage
///
/// It is responsible for loading and storing the settings in the D-Bus service.
pub struct Store<'a> {
    users_client: UsersClient<'a>,
    software_client: SoftwareClient<'a>
}

impl<'a> Store<'a> {
    pub fn new() -> Result<Self, zbus::Error> {
        Ok(Self {
            users_client: UsersClient::new(super::connection(None)?)?,
            software_client: SoftwareClient::new(super::connection(None)?)?
        })
    }

    /// Loads the installation settings from the D-Bus service
    pub fn load(&self) -> Result<Settings, Box<dyn Error>> {
        let first_user = self.users_client.first_user()?;
        let product = self.software_client.product()?;

        let settings = Settings {
            software: SoftwareSettings { product },
            user: UserSettings {
                user_name: first_user.user_name,
                autologin: first_user.autologin,
                full_name: first_user.full_name,
                password: first_user.password,
            },
        };
        Ok(settings)
    }

    /// Stores the given installation settings in the D-Bus service
    pub fn store(&self, settings: &Settings) -> Result<(), Box<dyn Error>> {
        // fixme: improve
        let first_user = FirstUser {
            user_name: settings.user.user_name.clone(),
            full_name: settings.user.full_name.clone(),
            autologin: settings.user.autologin,
            password: settings.user.password.clone(),
            ..Default::default()
        };
        dbg!(&settings);
        self.software_client.select_product(&settings.software.product)?;
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
        if let Some((ns, id)) = s.split_once('.') {
            return Ok(Self(ns.to_string(), id.to_string()));
        }
        Err(format!("not a valid configuration key: {s}"))
    }
}
