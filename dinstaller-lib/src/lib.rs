pub mod software;
pub mod storage;
pub mod users;
// TODO: maybe expose only clients when we have it?
pub mod proxies;

use std::path::Path;

pub(crate) fn connection() -> Result<zbus::blocking::Connection, zbus::Error>{
    let path = if Path::new("/run/d-installer/bus").exists() {
        "/run/d-installer/bus"
    } else {
        "/run/dbus/system_bus_socket"
    };
    let address = format!("unix:path={}", path);
    zbus::blocking::ConnectionBuilder::address(address.as_str())?.build()
}

macro_rules! validation_struct {
    ($proxy:ident) => {
        pub fn validate(&self) -> zbus::Result<Result<(), Vec<String>>> {
            let valid = self.$proxy.valid()?;
            let errors = self.$proxy.errors()?;
            
            if valid {
                Ok(Ok(()))
            } else {
                Ok(Err(errors))
            }
        }
    };
}

pub(crate) use validation_struct;

macro_rules! validation_method {
    ($client:ident) => {
        pub fn validate() -> zbus::Result<Result<(), Vec<String>>> {
            let client = $client::new(super::connection()?)?;
            client.validate()
        }
    };
}

pub(crate) use validation_method;