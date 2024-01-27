use serde_json::Value;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;

#[derive(Deserialize)]
pub struct Config {
    pub directory_path: String,
    pub pattern: String,
    pub output_directory: String,
    pub output_format: Option<Value>,
}

pub fn read_config(file_path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let mut config_str = String::new();
    let mut file = File::open(file_path)?;
    file.read_to_string(&mut config_str)?;
    let config = toml::from_str(&config_str)?;
    Ok(config)
}
