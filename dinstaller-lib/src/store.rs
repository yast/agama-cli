use zbus::export::futures_util::future::try_join;

use crate::install_settings::{InstallSettings, SoftwareSettings, StorageSettings, UserSettings};
use crate::software::SoftwareClient;
use crate::storage::StorageClient;
use crate::users::{FirstUser, UsersClient};
use std::{default::Default, error::Error};

/// Loading and storing the settings in the D-Bus service
///
/// This struct uses the default connection built by [connection function](super::connection).
pub struct Store<'a> {
    users_client: UsersClient<'a>,
    software_client: SoftwareClient<'a>,
    storage_client: StorageClient<'a>,
}

impl<'a> Store<'a> {
    pub async fn new() -> Result<Store<'a>, zbus::Error> {
        Ok(Self {
            users_client: UsersClient::new(super::connection().await?).await?,
            software_client: SoftwareClient::new(super::connection().await?).await?,
            storage_client: StorageClient::new(super::connection().await?).await?,
        })
    }

    /// Loads the installation settings from the D-Bus service
    pub async fn load(&self) -> Result<InstallSettings, Box<dyn Error>> {
        let (first_user, product) = try_join(
            self.users_client.first_user(),
            self.software_client.product()
        ).await?;

        let settings = InstallSettings {
            storage: Default::default(),
            software: SoftwareSettings {
                product: Some(product),
            },
            user: UserSettings {
                user_name: Some(first_user.user_name),
                autologin: Some(first_user.autologin),
                full_name: Some(first_user.full_name),
                password: Some(first_user.password),
            },
        };
        Ok(settings)
    }

    /// Stores the given installation settings in the D-Bus service
    pub async fn store(&self, settings: &InstallSettings) -> Result<(), Box<dyn Error>> {
        // be on safe side for storing settings here, but probably user and storage can be
        // parallel. With software product can be issue
        self.store_software_settings(&settings.software).await?;
        self.store_user_settings(&settings.user).await?;
        self.store_storage_settings(&settings.storage).await?;
        Ok(())
    }

    async fn store_user_settings(&self, settings: &UserSettings) -> Result<(), Box<dyn Error>> {
        // fixme: improve
        let first_user = FirstUser {
            user_name: settings.user_name.clone().unwrap_or_default(),
            full_name: settings.full_name.clone().unwrap_or_default(),
            autologin: settings.autologin.unwrap_or_default(),
            password: settings.password.clone().unwrap_or_default(),
            ..Default::default()
        };
        self.users_client.set_first_user(&first_user).await?;
        Ok(())
    }

    async fn store_software_settings(&self, settings: &SoftwareSettings) -> Result<(), Box<dyn Error>> {
        if let Some(product) = &settings.product {
            self.software_client.select_product(product).await?;
        }
        Ok(())
    }

    async fn store_storage_settings(&self, settings: &StorageSettings) -> Result<(), Box<dyn Error>> {
        self.storage_client.calculate(
            settings.devices.iter().map(|d| d.name.clone()).collect(),
            settings.encryption_password.clone().unwrap_or_default(),
            settings.lvm.unwrap_or_default(),
        ).await?;
        // TODO: convert the returned value to an error
        Ok(())
    }
}
