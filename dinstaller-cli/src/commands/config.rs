use std::str::FromStr;

/// Represents a configuration key (e.g., "storage.lvm")
#[derive(Debug)]
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

// this type is not needed at all
#[derive(Debug)]
pub struct ConfigValue(String);

#[derive(Debug)]
pub struct ConfigAssignment(ConfigKey, ConfigValue);

impl FromStr for ConfigAssignment {
    type Err = String; // fixme: use a real error

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((key, value)) = s.split_once("=") {
          if let Ok(key) = ConfigKey::from_str(key) {
                return Ok(ConfigAssignment(key, ConfigValue(value.to_string())))
            }
        }

        Err("Not a valid assignment".to_string())
    }
}

#[derive(Debug)]
pub enum ConfigAction {
    Add(ConfigKey, ConfigValue),
    Set(Vec<ConfigAssignment>),
    Reset(Vec<ConfigKey>),
    Show(Vec<ConfigKey>)
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
    fn test_config_assignment_from_str() {
        let ConfigAssignment(key, value) = ConfigAssignment::from_str("storage.lvm=true").unwrap();
        let ConfigKey(namespace, name) = key;
        assert_eq!(namespace, "storage".to_string());
        assert_eq!(name, "lvm".to_string());
        assert_eq!(value.0, "true".to_string());

        assert_eq!(ConfigKey::from_str("storage=lvm-true").is_err(), true);
    }
}

