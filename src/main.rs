extern crate image;
extern crate regex;
extern crate serde_json;
extern crate serde;
extern crate toml;
extern crate walkdir;

mod config;
mod errors;
mod file_processing;
mod image_processing;
mod utils;

use config::read_config;
use file_processing::{process_matching_files, write_json};
use image_processing::{find_matching_files};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = read_config("config.toml")?;
    let matching_files = find_matching_files(&config.directory_path, &config.pattern)?;
    let file_descriptions = process_matching_files(&matching_files, &config)?;

    write_json(&matching_files, &config.output_directory, "matching_files.json")?;
    write_json(&file_descriptions, &config.output_directory, "file_descriptions.json")?;

    Ok(())
}
