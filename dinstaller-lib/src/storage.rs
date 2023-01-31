use super::proxies::{StorageProposalProxy,Storage1Proxy,CalculatorProxy};
use zbus::blocking::{Connection, ConnectionBuilder};
use serde::Serialize;

pub struct StorageClient<'a> {
    pub connection: Connection,
    calculator_proxy: CalculatorProxy<'a>,
    storage_proxy: Storage1Proxy<'a>,
}

impl<'a> StorageClient<'a> {
    pub fn new(connection: Connection) -> zbus::Result<Self> {
        Ok(Self { 
            calculator_proxy: CalculatorProxy::new(&connection)?,
            storage_proxy: Storage1Proxy::new(&connection)?,
            connection,
        })
    }

    /// Returns the proposal proxy
    ///
    /// The proposal might not exist.
    // NOTE: should we implement some kind of memoization?
    fn proposal_proxy(&self) -> zbus::Result<StorageProposalProxy<'a>> {
        StorageProposalProxy::new(&self.connection)
    }

    /// Returns the available devices
    ///
    /// These devices can be used for installing the system.
    pub fn available_devices(&self) -> zbus::Result<Vec<StorageDevice>> {
        let devices: Vec<_> = self.calculator_proxy.available_devices()?
            .into_iter()
            .map(|(name, description, _)| StorageDevice { name, description })
            .collect();
        Ok(devices)
    }

    /// Returns the candidate devices for the proposal
    pub fn candidate_devices(&self) -> zbus::Result<Vec<String>> {
        self.proposal_proxy()?.candidate_devices()
    }

    /// Runs the probing process
    pub fn probe(&self) -> zbus::Result<()> {
        self.storage_proxy.probe()
    }
}

pub fn available_devices() -> zbus::Result<Vec<StorageDevice>> {
    let connection = ConnectionBuilder::address("unix:path=/run/d-installer/bus").unwrap()
        .build().unwrap();
    let client = StorageClient::new(connection)?;
    client.available_devices()
}

pub fn candidate_devices() -> zbus::Result<Vec<String>> {
    let connection = ConnectionBuilder::address("unix:path=/run/d-installer/bus").unwrap()
        .build().unwrap();
    let client = StorageClient::new(connection)?;
    client.candidate_devices()
}

#[derive(Serialize, Debug)]
pub struct StorageDevice {
    name: String,
    description: String
}
