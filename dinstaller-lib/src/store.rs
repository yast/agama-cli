mod software;
mod storage;
mod users;

use crate::install_settings::{InstallSettings, Scope};
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
    pub fn load(&self, only: Option<Vec<Scope>>) -> Result<InstallSettings, Box<dyn Error>> {
        let scopes = match only {
            Some(scopes) => scopes,
            None => Scope::all().to_vec(),
        };

        let mut settings: InstallSettings = Default::default();
        if scopes.contains(&Scope::Storage) {
            settings.storage = Some(self.storage.load()?);
        }

        if scopes.contains(&Scope::Software) {
            settings.software = Some(self.software.load()?);
        }

        if scopes.contains(&Scope::Users) {
            settings.user = Some(self.users.load()?);
        }
        Ok(settings)
    }

    /// Stores the given installation settings in the D-Bus service
    pub fn store(&self, settings: &InstallSettings) -> Result<(), Box<dyn Error>> {
        if let Some(software) = &settings.software {
            self.software.store(software)?;
        }
        if let Some(user) = &settings.user {
            self.users.store(user)?;
        }
        if let Some(storage) = &settings.storage {
            self.storage.store(storage)?;
        }
        Ok(())
    }
}
