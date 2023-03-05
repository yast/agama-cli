use crate::install_settings::{InstallSettings, SoftwareSettings, UserSettings};
use crate::software::SoftwareClient;
use crate::users::{FirstUser, UsersClient};
use std::{default::Default, error::Error};

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
    pub fn load(&self) -> Result<InstallSettings, Box<dyn Error>> {
        let first_user = self.users_client.first_user()?;
        let product = self.software_client.product()?;

        let settings = InstallSettings {
            storage: Default::default(),
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
    pub fn store(&self, settings: &InstallSettings) -> Result<(), Box<dyn Error>> {
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
