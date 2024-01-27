use crate::errors::AppError;
use crate::utils::is_image_file;
use image::{RgbaImage, GenericImage};
use regex::Regex;
use serde_json::Value;
use std::collections::HashMap;
use walkdir::WalkDir;

pub const IMAGE_WIDTH: u32 = 512;
pub const IMAGE_HEIGHT: u32 = 512;

pub type FileNamePair = (String, String);
pub type GroupMap = HashMap<String, HashMap<String, HashMap<String, Vec<FileNamePair>>>>;

pub fn find_matching_files(directory: &str, pattern: &str) -> Result<GroupMap, AppError> {
    let re = Regex::new(pattern).map_err(|e| AppError::Other(e.to_string()))?;
    let mut matching_files = GroupMap::new();

    for entry in WalkDir::new(directory).into_iter().filter_map(Result::ok).filter(is_image_file) {
        let file_name = entry.file_name().to_string_lossy();
        if let Some(cap) = re.captures(&file_name) {
            let group_values = (1..=4)
                .map(|i| cap.get(i).unwrap().as_str().to_string())
                .collect::<Vec<_>>();
            let full_filename = entry.path().to_string_lossy().into_owned();

            matching_files.entry(group_values[1].clone())
                .or_default()
                .entry(group_values[2].clone())
                .or_default()
                .entry(group_values[3].clone())
                .or_default()
                .push((group_values[0].clone(), full_filename));
        }
    }

    Ok(matching_files)
}

pub fn create_output(group1_filename_list: &[(String, String)], output_format: &Option<Value>) -> Vec<Vec<String>> {
    match output_format {
        Some(Value::Array(structure)) => structure.iter().map(|item| process_item(item, group1_filename_list)).collect(),
        _ => {
            println!("No valid output format found");
            vec![]
        }
    }
}

fn process_item(item: &Value, group1_filename_list: &[(String, String)]) -> Vec<String> {
    match item {
        Value::String(s) => {
            vec![find_filename_for_prefix(s, group1_filename_list)]
        },
        Value::Array(row) => {
            row.iter()
               .filter_map(|item| match item {
                   Value::String(s) => Some(find_filename_for_prefix(s, group1_filename_list)),
                   _ => None
               })
               .collect()
        },
        _ => vec![]
    }
}

fn find_filename_for_prefix(prefix: &str, group1_filename_list: &[(String, String)]) -> String {
    group1_filename_list.iter()
        .find(|(p, _)| p == prefix)
        .map(|(_, filename)| filename.clone())
        .unwrap_or_else(String::new)
}

pub fn concatenate_images(output: &[Vec<String>], image_width: u32, image_height: u32) -> Result<RgbaImage, AppError> {
    let (num_rows, num_cols) = get_dimensions(output);
    if num_rows == 0 || num_cols == 0 {
        return Err(AppError::ImageProcessing("No images to concatenate".into()));
    }


    let (total_width, total_height) = calculate_total_dimensions(num_rows, num_cols, image_width, image_height);
    let mut new_image = RgbaImage::new(total_width, total_height);

    for (y, row) in output.iter().enumerate() {
        for (x, filename) in row.iter().enumerate() {
            add_image_to_canvas(&mut new_image, filename, x, y, image_width, image_height)?;
        }
    }

    Ok(new_image)
}

fn get_dimensions(output: &[Vec<String>]) -> (usize, usize) {
    let num_rows = output.len();
    let num_cols = output.first().map_or(0, |row| row.len());
    (num_rows, num_cols)
}

fn calculate_total_dimensions(num_rows: usize, num_cols: usize, image_width: u32, image_height: u32) -> (u32, u32) {
    if num_rows == 1 {
        (num_cols as u32 * image_width, image_height)
    } else if num_cols == 1 {
        (image_width, num_rows as u32 * image_height)
    } else {
        (num_cols as u32 * image_width, num_rows as u32 * image_height)
    }
}

fn add_image_to_canvas(canvas: &mut RgbaImage, filename: &str, x: usize, y: usize, image_width: u32, image_height: u32) -> Result<(), AppError> {
    if !filename.is_empty() {
        let img = image::open(filename).map_err(AppError::Image)?;
        let img = img.to_rgba8();
        let pos_x = x as u32 * image_width;
        let pos_y = y as u32 * image_height;
        canvas.copy_from(&img, pos_x, pos_y).map_err(|_| AppError::ImageProcessing("Error copying image".into()))?;
    }
    Ok(())
}
