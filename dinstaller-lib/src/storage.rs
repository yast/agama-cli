use super::proxies::{StorageProposalProxy,Storage1Proxy,CalculatorProxy};
use std::collections::HashMap;
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

    pub fn from_address(address: &str) -> zbus::Result<Self> {
        let connection = ConnectionBuilder::address(address)?.build()?;
        Self::new(connection)
    }

    pub fn from_default_address() -> zbus::Result<Self> {
        Self::from_address("unix:path=/run/d-installer/bus")
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

    /// Returns the storage proposal
    pub fn proposal(&self) -> zbus::Result<StorageProposal> {
        let proxy = self.proposal_proxy()?;

        Ok(StorageProposal {
            candidate_devices: proxy.candidate_devices().unwrap_or(vec![]),
            encryption_password: proxy.encryption_password().ok(),
            lvm: proxy.lvm().unwrap_or(false),
        })
    }

    /// Asks the storage service to calculate a new proposal
    ///
    // FIXME: this function should return a `StorageProposalArgs` or just directly the HashMap
    pub fn calculate_proposal(&self, proposal: StorageProposal) -> zbus::Result<u32> {
        let mut settings: HashMap<&str, zbus::zvariant::Value> = HashMap::from([
            ("CandidateDevices", proposal.candidate_devices.into()),
            ("LVM", proposal.lvm.into()),
        ]);

        if let Some(password) = proposal.encryption_password {
            settings.insert("EncryptionPassword", password.into());
        }

        self.calculator_proxy.calculate(settings)
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

pub fn proposal() -> zbus::Result<StorageProposal> {
    let connection = ConnectionBuilder::address("unix:path=/run/d-installer/bus").unwrap()
        .build().unwrap();
    let client = StorageClient::new(connection)?;
    client.proposal()
}

#[derive(Serialize, Debug)]
pub struct StorageDevice {
    name: String,
    description: String
}

/// Represents the result of an storage proposal
///
/// FIXME: by now, we are using the `StorageProposal` struct in `proposal`
/// and `calculate_proposal`. But they are different things!
#[derive(Debug, Serialize)]
pub struct StorageProposal {
    pub candidate_devices: Vec<String>,
    pub lvm: bool,
    pub encryption_password: Option<String>
}
