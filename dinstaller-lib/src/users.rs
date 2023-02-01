use super::proxies::Users1Proxy;
use zbus::blocking::Connection;
use serde::Serialize;

pub struct UsersClient<'a> {
    users_proxy: Users1Proxy<'a>,
}

#[derive(Serialize, Debug)]
pub struct FirstUser {
    pub full_name: String,
    pub user_name: String,
    pub autologin: bool,
    pub data: std::collections::HashMap<String, zbus::zvariant::OwnedValue>
}

impl FirstUser {
    fn from_dbus(dbus_data: zbus::Result<(
        String,
        String,
        bool,
        std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
    )>) -> zbus::Result<Self> {
        let data = dbus_data?;
        Ok(
            Self { 
                full_name: data.0,
                user_name: data.1,
                autologin: data.2,
                data: data.3
            }
        )
    }
}

impl<'a> UsersClient<'a> {
    pub fn new(connection: Connection) -> zbus::Result<Self> {
        Ok(Self { 
            users_proxy: Users1Proxy::new(&connection)?
        })
    }

    /// Returns the settings for first non admin user
    pub fn first_user(&self) -> zbus::Result<FirstUser> {
        FirstUser::from_dbus(self.users_proxy.first_user())
    }

    pub fn is_root_password(&self) -> zbus::Result<bool> {
        self.users_proxy.root_password_set()
    }

    pub fn root_ssh_key(&self) -> zbus::Result<String> {
        self.users_proxy.root_sshkey()
    }
}

pub fn first_user() -> zbus::Result<FirstUser> {
    let client = UsersClient::new(super::connection()?)?;
    client.first_user()
}

pub fn is_root_password() -> zbus::Result<bool> {
    let client = UsersClient::new(super::connection()?)?;
    client.is_root_password()
}

pub fn root_ssh_key() -> zbus::Result<String> {
    let client = UsersClient::new(super::connection()?)?;
    client.root_ssh_key()
}