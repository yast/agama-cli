use std::collections::HashMap;

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

    pub fn set_first_user(&self, user: &FirstUser, password: &str) -> zbus::Result<Result<(),Vec<String>>> {
        let result = self.users_proxy.set_first_user(
            &user.full_name,
            &user.user_name,
            password,
            user.autologin,
            HashMap::new())?; // data not used yet, needs transformation
        if result.0 {
            Ok(Ok(()))
        } else {
            Ok(Err(result.1))
        }
        
    }

    pub fn remove_first_user(&self) -> zbus::Result<u32> {
        self.users_proxy.remove_first_user()
    }

    pub fn is_root_password(&self) -> zbus::Result<bool> {
        self.users_proxy.root_password_set()
    }

    pub fn root_ssh_key(&self) -> zbus::Result<String> {
        self.users_proxy.root_sshkey()
    }

    pub fn set_root_password(&self, value: &str, encrypted: bool) -> zbus::Result<u32> {
        self.users_proxy.set_root_password(value, encrypted)
    }

    pub fn remove_root_password(&self) -> zbus::Result<u32> {
        self.users_proxy.remove_root_password()
    }

    pub fn set_root_ssh_key(&self, value: &str) -> zbus::Result<u32> {
        self.users_proxy.set_root_sshkey(value)
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

pub fn set_root_password(value: &str, encrypted: bool) -> zbus::Result<u32> {
    let client = UsersClient::new(super::connection()?)?;
    client.set_root_password(value, encrypted)
}