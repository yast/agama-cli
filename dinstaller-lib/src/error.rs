use curl;
use serde_json;
use std::io;
use thiserror::Error;
use zbus;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("D-Bus service error: {0}")]
    DBus(#[from] zbus::Error),
    #[error("Input/output error: {0}")]
    InputOutputError(#[from] io::Error),
}

#[derive(Error, Debug)]
pub enum ProfileError {
    #[error("Cannot read the profile '{0}'")]
    Unreachable(#[from] curl::Error),
    #[error("No hardware information available: '{0}'")]
    NoHardwareInfo(io::Error),
    #[error("Could not evaluate the profile: '{0}'")]
    EvaluationError(io::Error),
    #[error("Input/output error: '{0}'")]
    InputOutputError(#[from] io::Error),
    #[error("The profile is not a valid JSON file")]
    FormatError(#[from] serde_json::Error),
}
