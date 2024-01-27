use walkdir::DirEntry;

pub fn is_image_file(entry: &DirEntry) -> bool {
    entry.file_type().is_file() && entry.file_name().to_string_lossy().ends_with(".png")
}
