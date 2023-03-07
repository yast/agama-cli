use curl::easy::Easy;
use jsonschema::JSONSchema;
use serde_json;
use std::error::Error;
use std::{
    fs,
    io::{stdout, Write},
    path::Path,
};

pub fn download(url: &str) -> Result<(), Box<dyn Error>> {
    let mut easy = Easy::new();
    easy.url(url)?;
    easy.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
    })?;
    easy.perform()?;
    Ok(())
}

pub enum ValidationResult {
    Valid,
    NotValid(Vec<String>),
}

pub struct ProfileValidator {
    schema: JSONSchema,
}

impl ProfileValidator {
    pub fn default_schema() -> Result<Self, Box<dyn Error>> {
        Self::new(Path::new("dinstaller-lib/share/profile.schema.json"))
    }

    pub fn new(schema_path: &Path) -> Result<Self, Box<dyn Error>> {
        let contents = fs::read_to_string(schema_path)?;
        let schema = serde_json::from_str(&contents)?;
        let schema = JSONSchema::compile(&schema).expect("A valid schema");
        Ok(Self { schema })
    }

    pub fn validate(&self, profile_path: &Path) -> Result<ValidationResult, Box<dyn Error>> {
        let contents = fs::read_to_string(profile_path)?;
        let profile = serde_json::from_str(&contents)?;
        let result = self.schema.validate(&profile);
        if let Err(errors) = result {
            let messages: Vec<String> = errors.map(|e| format!("{e}")).collect();
            return Ok(ValidationResult::NotValid(messages));
        }
        Ok(ValidationResult::Valid)
    }
}
