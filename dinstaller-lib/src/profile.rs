use curl::easy::Easy;
use std::error::Error;
use std::io::{stdout, Write};
use std::path::Path;

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
