mod software;
mod users;

use crate::install_settings::{InstallSettings, StorageSettings};
use crate::storage::StorageClient;
use crate::store::software::SoftwareStore;
use crate::store::users::UsersStore;
use std::{default::Default, error::Error};

/// Loading and storing the settings in the D-Bus service
///
/// This struct uses the default connection built by [connection function](super::connection).
pub struct Store<'a> {
    users: UsersStore<'a>,
    software: SoftwareStore<'a>,
    storage_client: StorageClient<'a>,
}

impl<'a> Store<'a> {
    pub fn new() -> Result<Self, zbus::Error> {
        Ok(Self {
            users: UsersStore::new(super::connection()?)?,
            software: SoftwareStore::new(super::connection()?)?,
            storage_client: StorageClient::new(super::connection()?)?,
        })
    }

    /// Loads the installation settings from the D-Bus service
    pub fn load(&self) -> Result<InstallSettings, Box<dyn Error>> {
        let settings = InstallSettings {
            storage: Default::default(),
            software: self.software.load()?,
            user: self.users.load()?,
        };
        Ok(settings)
    }

    /// Stores the given installation settings in the D-Bus service
    pub fn store(&self, settings: &InstallSettings) -> Result<(), Box<dyn Error>> {
        self.software.store(&settings.software)?;
        self.users.store(&settings.user)?;
        self.store_storage_settings(&settings.storage)?;
        Ok(())
    }

    fn store_storage_settings(&self, settings: &StorageSettings) -> Result<(), Box<dyn Error>> {
        self.storage_client.calculate(
            settings.devices.iter().map(|d| d.name.clone()).collect(),
            settings.encryption_password.clone().unwrap_or_default(),
            settings.lvm.unwrap_or_default(),
        )?;
        // TODO: convert the returned value to an error
        Ok(())
    }
}
