pub mod install_settings;
pub mod manager;
pub mod profile;
pub mod settings;
pub mod software;
pub mod storage;
pub mod users;
// TODO: maybe expose only clients when we have it?
pub mod progress;
pub mod proxies;
mod store;
use async_std::task::block_on;
pub use store::Store;

use std::path::Path;

pub fn connection() -> zbus::Connection {
    let path = if Path::new("/run/d-installer/bus").exists() {
        "/run/d-installer/bus"
    } else {
        "/run/dbus/system_bus_socket"
    };
    let address = format!("unix:path={path}");
    block_on(
        zbus::ConnectionBuilder::address(address.as_str())
            .expect("Failed to create D-Bus connection").build()
    ).expect("Failed to create D-Bus connection")
}
