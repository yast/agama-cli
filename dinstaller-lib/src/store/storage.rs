use crate::install_settings::StorageSettings;
use crate::storage::StorageClient;
use std::default::Default;
use std::error::Error;
use zbus::blocking::Connection;

/// Loads and stores the storage settings from/to the D-Bus service.
pub struct StorageStore<'a> {
    storage_client: StorageClient<'a>,
}

impl<'a> StorageStore<'a> {
    pub fn new(connection: Connection) -> Result<Self, zbus::Error> {
        Ok(Self {
            storage_client: StorageClient::new(connection)?,
        })
    }

    // TODO: read the settings from the service
    pub fn load(&self) -> Result<StorageSettings, Box<dyn Error>> {
        Ok(Default::default())
    }

    pub fn store(&self, settings: &StorageSettings) -> Result<(), Box<dyn Error>> {
        self.storage_client.calculate(
            settings.devices.iter().map(|d| d.name.clone()).collect(),
            settings.encryption_password.clone().unwrap_or_default(),
            settings.lvm.unwrap_or_default(),
        )?;
        Ok(())
    }
}
