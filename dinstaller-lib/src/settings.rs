//! Configuration settings handling
//!
//! This module implements the mechanisms to load and store the installation settings.
use crate::attributes::{AttributeValue, Attributes};
use crate::software::SoftwareClient;
use crate::users::{FirstUser, UsersClient};
use dinstaller_derive::DInstallerAttributes;
use serde::Serialize;
use std::{default::Default, error::Error};

/// Installation settings
///
/// This struct represents installation settings. It serves as an entry point and it is composed of
/// other structs which hold the settings for each area ("users", "software", etc.).
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

/// User settings
///
/// Holds the user settings for the installation.
#[derive(Debug, Default, DInstallerAttributes, Serialize)]
pub struct UserSettings {
    /// First user's full name
    pub full_name: String,
    /// First user's username
    pub user_name: String,
    /// First user's password (in clear text)
    pub password: String,
    /// Whether auto-login should enabled or not
    pub autologin: bool,
}

/// Storage settings for installation
#[derive(Debug, DInstallerAttributes, Serialize)]
pub struct StorageSettings {
    /// Whether LVM should be enabled
    lvm: bool,
    /// Encryption password for the storage devices (in clear text)
    encryption_password: String,
}

/// Software settings for installation
#[derive(Debug, Default, DInstallerAttributes, Serialize)]
pub struct SoftwareSettings {
    /// ID of the product to install (e.g., "ALP", "Tumbleweed", etc.)
    product: String,
}

/// Loading and storing the settings in the D-Bus service
///
/// This struct uses the default connection built by [connection function](super::connection).
pub struct Store<'a> {
    users_client: UsersClient<'a>,
    software_client: SoftwareClient<'a>,
}

impl<'a> Store<'a> {
    pub fn new() -> Result<Self, zbus::Error> {
        Ok(Self {
            users_client: UsersClient::new(super::connection()?)?,
            software_client: SoftwareClient::new(super::connection()?)?,
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
        self.software_client
            .select_product(&settings.software.product)?;
        self.users_client.set_first_user(&first_user)?;
        Ok(())
    }
}
