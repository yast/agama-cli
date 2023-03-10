use crate::install_settings::UserSettings;
use crate::users::{FirstUser, UsersClient};
use std::error::Error;
use zbus::Connection;

/// Loads and stores the users settings from/to the D-Bus service.
pub struct UsersStore<'a> {
    users_client: UsersClient<'a>,
}

impl<'a> UsersStore<'a> {
    pub async fn new(connection: Connection) -> Result<UsersStore<'a>, zbus::Error> {
        Ok(Self {
            users_client: UsersClient::new(connection).await?,
        })
    }

    pub async fn load(&self) -> Result<UserSettings, Box<dyn Error>> {
        let first_user = self.users_client.first_user().await?;
        Ok(UserSettings {
            user_name: Some(first_user.user_name),
            autologin: Some(first_user.autologin),
            full_name: Some(first_user.full_name),
            password: Some(first_user.password),
        })
    }

    pub async fn store(&self, settings: &UserSettings) -> Result<(), Box<dyn Error>> {
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
}
