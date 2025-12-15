//! SDK error types

use thiserror::Error;

/// SDK errors
#[derive(Error, Debug)]
pub enum SdkError {
    #[error("Invalid storyboard: {0}")]
    InvalidStoryboard(String),

    #[error("Story Protocol error: {0}")]
    StoryProtocol(String),

    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Validation error: {0}")]
    Validation(String),
}
