use std::str::FromStr;
use std::error::Error;
use dinstaller_lib::storage::StorageClient;
use std::collections::HashMap;

/// Represents a configuration key (e.g., "storage.lvm")
#[derive(Debug,PartialEq, Eq, Hash)]
pub struct ConfigKey(String, String);

impl FromStr for ConfigKey {
    type Err = String; // fixme: use a real error

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((namespace, name)) = s.split_once(".") {
            return Ok(ConfigKey(namespace.to_string(), name.to_string()))
        }

        Err("Not a valid key".to_string())
    }
}

/// Represents a change to a configuration key, like setting a value, adding an item to a
/// collection, etc.
#[derive(Debug)]
pub enum ConfigAction {
    // Add(ConfigKey, String),
    // Reset(Vec<ConfigKey>),
    Set(HashMap<ConfigKey, String>),
    Show(Vec<ConfigKey>)
}

/// Runs a configuration action
///
/// Each configuration action cannot run in an isolated way, as many of them might be related.
pub struct StorageActionsRunner<'a> {
    storage_client: StorageClient<'a>
}

impl<'a> StorageActionsRunner<'a> {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let storage_client = StorageClient::from_default_address()?;
        Ok(Self { storage_client })
    }

    pub fn run(&self, action: ConfigAction) -> Result<(), Box<dyn Error>> {
        match &action {
            ConfigAction::Set(changes) => {
                self.calculate_proposal(changes)?;
                Ok(())
            },
            _ => unimplemented!()
        }
    }

    fn calculate_proposal(&self, changes: &HashMap<ConfigKey, String>) -> Result<(), Box<dyn Error>> {
        let mut proposal = self.storage_client.proposal()?;

        if let Some(value) = changes.get(&ConfigKey::from_str("storage.lvm").unwrap()) {
            proposal.lvm = value == "true";
        }

        if let Some(value) = changes.get(&ConfigKey::from_str("storage.encryption_password").unwrap()) {
            proposal.encryption_password = Some(value.to_string());
        }

        self.storage_client.calculate_proposal(proposal)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_config_key_from_str() {
        let ConfigKey(namespace, name) = ConfigKey::from_str("storage.lvm").unwrap();
        assert_eq!(namespace, "storage".to_string());
        assert_eq!(name, "lvm".to_string());

        assert_eq!(ConfigKey::from_str("storage-lvm").is_err(), true);
    }

    #[test]
    fn test_hash() {
        let mut changes: HashMap<ConfigKey, String> = HashMap::new();
        changes.insert(
            ConfigKey("storage".to_string(), "lvm".to_string()), "Foo".to_string()
        );
    }
}
