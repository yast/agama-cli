pub mod error;
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
pub mod questions;
mod store;
pub use store::Store;

use crate::error::ServiceError;
use std::path::Path;

pub async fn connection() -> Result<zbus::Connection, ServiceError> {
    let path = if Path::new("/run/d-installer/bus").exists() {
        "/run/d-installer/bus"
    } else {
        "/run/dbus/system_bus_socket"
    };
    let address = format!("unix:path={path}");
    let conn = zbus::ConnectionBuilder::address(address.as_str())?
        .build()
        .await?;
    Ok(conn)
}
