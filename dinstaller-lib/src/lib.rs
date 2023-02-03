pub mod attributes;
pub mod software;
pub mod storage;
pub mod users;
mod proxies;
pub mod settings;

use std::fs;
use zbus::blocking::{Connection, ConnectionBuilder};

pub fn connection(address: Option<String>) -> Result<Connection, zbus::Error> {
    let bus_address = match address {
        Some(address) => address,
        None => find_bus_address()
    };

    println!("using {}", bus_address);
    ConnectionBuilder::address(bus_address.as_str())?.build()
}

const DBUS_ADDRESS_FILE: &str = "/run/d-installer/bus.address";

fn find_bus_address() -> String {
    if let Ok(contents) = fs::read_to_string(DBUS_ADDRESS_FILE) {
        return contents;
    }

    format!("unix:path=/run/dbus/system_bus_socket")
}

