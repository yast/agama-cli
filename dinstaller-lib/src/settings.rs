//! Configuration settings handling
//!
//! This module implements the mechanisms to load and store the installation settings.
use crate::attributes::{AttributeValue, Attributes};
use dinstaller_derive::DInstallerAttributes;
use serde::Serialize;
use std::default::Default;

/// Installation settings
///
/// This struct represents installation settings. It serves as an entry point and it is composed of
/// other structs which hold the settings for each area ("users", "software", etc.).
#[derive(Debug, Default, Serialize)]
pub struct Settings {
    pub user: UserSettings,
    pub software: SoftwareSettings,
    pub storage: StorageSettings,
}

impl Attributes for Settings {
    fn add(&mut self, attr: &str, value: AttributeValue) -> Result<(), &'static str> {
        if let Some((ns, id)) = attr.split_once('.') {
            match ns {
                "software" => self.software.add(id, value)?,
                "user" => self.user.add(id, value)?,
                "storage" => self.storage.add(id, value)?,
                _ => return Err("unknown attribute"),
            }
        }
        Ok(())
    }

    fn set(&mut self, attr: &str, value: AttributeValue) -> Result<(), &'static str> {
        if let Some((ns, id)) = attr.split_once('.') {
            match ns {
                "software" => self.software.set(id, value)?,
                "user" => self.user.set(id, value)?,
                "storage" => self.storage.set(id, value)?,
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
#[derive(Debug, Default, DInstallerAttributes, Serialize)]
pub struct StorageSettings {
    /// Whether LVM should be enabled
    pub lvm: bool,
    /// Encryption password for the storage devices (in clear text)
    pub encryption_password: String,
    /// Devices to use in the installation
    #[collection]
    pub devices: Vec<String>,
}

pub struct Device(String);

/// Software settings for installation
#[derive(Debug, Default, DInstallerAttributes, Serialize)]
pub struct SoftwareSettings {
    /// ID of the product to install (e.g., "ALP", "Tumbleweed", etc.)
    pub product: String,
}
