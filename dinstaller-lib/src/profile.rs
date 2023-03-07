use crate::manager::ManagerClient;
use curl::easy::Easy;
use jsonschema::JSONSchema;
use serde_json;
use std::{
    error::Error,
    fs, io,
    io::{stdout, Write},
    path::Path,
    process::Command,
};
use tempfile::tempdir;

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
        let relative_path = Path::new("dinstaller-lib/share/profile.schema.json");
        let path = if relative_path.exists() {
            relative_path
        } else {
            Path::new("/usr/share/dinstaller-rs/profile.schema.json")
        };
        Self::new(path)
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

pub struct ProfileEvaluator<'a> {
    manager_client: ManagerClient<'a>,
}

impl<'a> ProfileEvaluator<'a> {
    pub async fn new() -> Result<ProfileEvaluator<'a>, zbus::Error> {
        let manager_client = ManagerClient::new(super::connection().await?).await?;
        Ok(Self { manager_client })
    }

    pub async fn evaluate(&self, profile_path: &Path) -> Result<(), Box<dyn Error>> {
        let dir = tempdir()?;

        let working_path = dir.path().join("profile.jsonnet");
        fs::copy(profile_path, working_path)?;

        let hwinfo_path = dir.path().join("dinstaller.libsonnet");
        let hwinfo = self.manager_client.hwinfo().await?;
        fs::write(hwinfo_path, serde_json::to_string(&hwinfo)?)?;

        let result = Command::new("/usr/bin/jsonnet")
            .arg("profile.jsonnet")
            .current_dir(&dir)
            .output()?;
        io::stdout().write_all(&result.stdout)?;
        Ok(())
    }
}
