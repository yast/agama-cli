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
mod store;
pub use store::Store;

use crate::error::ServiceError;
use std::path::Path;
use std::io::{Error, ErrorKind};


pub async fn connection() -> Result<zbus::Connection, ServiceError> {
    let path = "/run/d-installer/bus";
    if !Path::new(path).exists() {
        let io_err = Error::new(ErrorKind::NotFound, format!("Bus socket {} does not exist", path));
        return Err(ServiceError::InputOutputError(io_err));
    }

    let address = format!("unix:path={path}");
    let conn = zbus::ConnectionBuilder::address(address.as_str())?
        .build()
        .await?;
    Ok(conn)
}
