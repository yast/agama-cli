use thiserror::Error;
use zbus;

#[derive(Error, Debug)]
#[error("D-Bus service error: {0}")]
pub struct ServiceError(#[from] zbus::Error);
