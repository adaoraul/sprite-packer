use crate::config::Config;
use crate::errors::AppError;
use crate::image_processing::{create_output, concatenate_images, IMAGE_WIDTH, IMAGE_HEIGHT};
use anyhow::{Result};
use rayon::prelude::*;
use serde::{Serialize};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub type GroupMap = HashMap<String, HashMap<String, HashMap<String, Vec<(String, String)>>>>;

pub fn process_matching_files(matching_files: &GroupMap, config: &Config) -> Result<HashMap<String, Vec<Vec<String>>>, AppError> {
    let file_descriptions: HashMap<_, _> = matching_files.par_iter().flat_map(|(group2, group3_list)| {
        group3_list.par_iter().flat_map(move |(group3, group4_data)| {
            group4_data.par_iter().map(move |(group4, group1_filename_list)| {
                let output = create_output(group1_filename_list, &config.output_format);
                let new_filename = format!("{}/{}/char_a_{}_{}_{}.png", config.output_directory, group2, group2, group3, group4);
                let path = PathBuf::from(&new_filename);
                if let Some(dir) = path.parent() {
                    fs::create_dir_all(dir)?;
                }

                let file_result = concatenate_images(&output, IMAGE_WIDTH, IMAGE_HEIGHT)
                    .map_err(|e| AppError::ImageProcessing(e.to_string()))
                    .and_then(|concatenated_image| {
                        concatenated_image.save(&path)?;
                        Ok((new_filename, output.iter().map(|row| row.to_vec()).collect::<Vec<_>>()))
                    });

                file_result
            })
        })
    }).collect::<Result<HashMap<_, _>, AppError>>()?;

    Ok(file_descriptions)
}

pub fn write_json<T: Serialize>(data: &T, output_directory: &str, file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(data)?;
    let file_path = format!("{}/{}", output_directory, file_name);
    let mut file = File::create(file_path)?;
    file.write_all(json.as_bytes())?;

    Ok(())
}
