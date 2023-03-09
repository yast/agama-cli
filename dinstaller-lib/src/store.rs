mod software;
mod storage;
mod users;

use crate::install_settings::InstallSettings;
use crate::store::software::SoftwareStore;
use crate::store::storage::StorageStore;
use crate::store::users::UsersStore;
use std::error::Error;

/// Loading and storing the settings in the D-Bus service
///
/// This struct uses the default connection built by [connection function](super::connection).
pub struct Store<'a> {
    users: UsersStore<'a>,
    software: SoftwareStore<'a>,
    storage: StorageStore<'a>,
}

impl<'a> Store<'a> {
    pub fn new() -> Result<Self, zbus::Error> {
        Ok(Self {
            users: UsersStore::new(super::connection()?)?,
            software: SoftwareStore::new(super::connection()?)?,
            storage: StorageStore::new(super::connection()?)?,
        })
    }

    /// Loads the installation settings from the D-Bus service
    pub fn load(&self) -> Result<InstallSettings, Box<dyn Error>> {
        let settings = InstallSettings {
            storage: self.storage.load()?,
            software: self.software.load()?,
            user: self.users.load()?,
        };
        Ok(settings)
    }

    /// Stores the given installation settings in the D-Bus service
    pub fn store(&self, settings: &InstallSettings) -> Result<(), Box<dyn Error>> {
        self.software.store(&settings.software)?;
        self.users.store(&settings.user)?;
        self.storage.store(&settings.storage)?;
        Ok(())
    }
}
