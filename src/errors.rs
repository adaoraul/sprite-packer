use image::ImageError;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    Io(std::io::Error),
    Image(ImageError),
    ImageProcessing(String),
    Other(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Io(e) => write!(f, "IO error: {}", e),
            AppError::Image(e) => write!(f, "Image error: {}", e),
            AppError::ImageProcessing(e) => write!(f, "Image processing error: {}", e),
            AppError::Other(e) => write!(f, "Other error: {}", e),
        }
    }
}

impl std::error::Error for AppError {}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> AppError {
        AppError::Io(err)
    }
}

impl From<ImageError> for AppError {
    fn from(err: ImageError) -> AppError {
        AppError::Image(err)
    }
}

impl From<String> for AppError {
    fn from(err: String) -> AppError {
        AppError::Other(err)
    }
}
