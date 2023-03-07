//! Configuration settings handling
//!
//! This module implements the mechanisms to load and store the installation settings.
use crate::settings::{SettingObject, SettingValue, Settings};
use dinstaller_derive::Settings;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::default::Default;

/// Installation settings
///
/// This struct represents installation settings. It serves as an entry point and it is composed of
/// other structs which hold the settings for each area ("users", "software", etc.).
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct InstallSettings {
    #[serde(default)]
    pub user: UserSettings,
    #[serde(default)]
    pub software: SoftwareSettings,
    #[serde(default)]
    pub storage: StorageSettings,
}

impl Settings for InstallSettings {
    fn add(&mut self, attr: &str, value: SettingObject) -> Result<(), &'static str> {
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

    fn set(&mut self, attr: &str, value: SettingValue) -> Result<(), &'static str> {
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

    fn merge(&mut self, other: Self) {
        self.software.merge(other.software);
        self.user.merge(other.user);
        self.storage.merge(other.storage);
    }
}

/// User settings
///
/// Holds the user settings for the installation.
#[derive(Debug, Default, Settings, Serialize, Deserialize)]
pub struct UserSettings {
    /// First user's full name
    pub full_name: Option<String>,
    /// First user's username
    pub user_name: Option<String>,
    /// First user's password (in clear text)
    pub password: Option<String>,
    /// Whether auto-login should enabled or not
    pub autologin: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge() {
        let mut user1 = UserSettings::default();
        let user2 = UserSettings {
            full_name: Some("Jane Doe".to_owned()),
            autologin: Some(true),
            ..Default::default()
        };
        user1.merge(user2);
        dbg!(user1);
    }
}

/// Storage settings for installation
#[derive(Debug, Default, Settings, Serialize, Deserialize)]
pub struct StorageSettings {
    /// Whether LVM should be enabled
    pub lvm: Option<bool>,
    /// Encryption password for the storage devices (in clear text)
    pub encryption_password: Option<String>,
    /// Devices to use in the installation
    #[collection_setting]
    pub devices: Vec<Device>,
}

/// Device to use in the installation
#[derive(Debug, Serialize, Deserialize)]
pub struct Device {
    /// Device name (e.g., "/dev/sda")
    pub name: String,
}

impl TryFrom<SettingObject> for Device {
    type Error = &'static str;

    fn try_from(value: SettingObject) -> Result<Self, Self::Error> {
        match value.0.get("name") {
            Some(name) => Ok(Device {
                name: name.clone().try_into()?,
            }),
            None => Err("'name' key not found"),
        }
    }
}

/// Software settings for installation
#[derive(Debug, Default, Settings, Serialize, Deserialize)]
pub struct SoftwareSettings {
    /// ID of the product to install (e.g., "ALP", "Tumbleweed", etc.)
    pub product: Option<String>,
}
