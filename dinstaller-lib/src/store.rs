mod users;

use crate::install_settings::{InstallSettings, SoftwareSettings, StorageSettings};
use crate::software::SoftwareClient;
use crate::storage::StorageClient;
use crate::store::users::UsersStore;
use std::{default::Default, error::Error};

/// Loading and storing the settings in the D-Bus service
///
/// This struct uses the default connection built by [connection function](super::connection).
pub struct Store<'a> {
    users: UsersStore<'a>,
    software_client: SoftwareClient<'a>,
    storage_client: StorageClient<'a>,
}

impl<'a> Store<'a> {
    pub fn new() -> Result<Self, zbus::Error> {
        Ok(Self {
            users: UsersStore::new(super::connection()?)?,
            software_client: SoftwareClient::new(super::connection()?)?,
            storage_client: StorageClient::new(super::connection()?)?,
        })
    }

    /// Loads the installation settings from the D-Bus service
    pub fn load(&self) -> Result<InstallSettings, Box<dyn Error>> {
        let product = self.software_client.product()?;

        let settings = InstallSettings {
            storage: Default::default(),
            software: SoftwareSettings {
                product: Some(product),
            },
            user: self.users.load()?,
        };
        Ok(settings)
    }

    /// Stores the given installation settings in the D-Bus service
    pub fn store(&self, settings: &InstallSettings) -> Result<(), Box<dyn Error>> {
        self.store_software_settings(&settings.software)?;
        self.users.store(&settings.user)?;
        self.store_storage_settings(&settings.storage)?;
        Ok(())
    }

    fn store_software_settings(&self, settings: &SoftwareSettings) -> Result<(), Box<dyn Error>> {
        if let Some(product) = &settings.product {
            self.software_client.select_product(product)?;
        }
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
