//! Storyboard JSON schema

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Storyboard - the main container for a video project
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Storyboard {
    /// Unique identifier
    pub id: String,
    /// Project title
    pub title: String,
    /// Optional description
    pub description: Option<String>,
    /// Video genre
    pub genre: Genre,
    /// Total duration in seconds
    pub duration: f64,
    /// Aspect ratio (e.g., "16:9", "9:16")
    pub aspect_ratio: String,
    /// Ordered list of layers
    pub layers: Vec<Layer>,
    /// Additional metadata
    pub metadata: StoryboardMetadata,
}

/// Video genre
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum Genre {
    Anime,
    Horror,
    Scifi,
    Commercial,
    Brainrot,
}

/// Layer in the storyboard
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Layer {
    /// Unique identifier
    pub id: String,
    /// Type of layer
    pub layer_type: LayerType,
    /// Position in the layer stack (0 = bottom)
    pub position: u32,
    /// Start time in seconds
    pub start_time: f64,
    /// Duration in seconds
    pub duration: f64,
    /// Asset associated with this layer
    pub asset: Asset,
}

/// Layer type
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum LayerType {
    Video,
    Audio,
    Text,
}

/// Asset data
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Asset {
    /// URL to the asset file
    pub url: Option<String>,
    /// Text content (for text layers)
    pub content: Option<String>,
    /// Prompt used to generate this asset
    pub generation_prompt: Option<String>,
    /// Story Protocol IP ID (if registered)
    pub ip_id: Option<String>,
    /// SHA-256 hash of the asset
    pub sha256_hash: Option<String>,
}

/// Storyboard metadata
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct StoryboardMetadata {
    /// Author/creator name
    pub author: Option<String>,
    /// Creation timestamp (ISO 8601)
    pub created_at: Option<String>,
    /// Tags for categorization
    pub tags: Vec<String>,
    /// License type
    pub license: Option<String>,
    /// Story Protocol IP ID for the entire storyboard
    pub ip_id: Option<String>,
}

impl Storyboard {
    /// Load storyboard from a JSON file
    pub fn from_file(path: &str) -> Result<Self, crate::SdkError> {
        let content = std::fs::read_to_string(path)
            .map_err(|_| crate::SdkError::FileNotFound(path.to_string()))?;
        let storyboard: Self = serde_json::from_str(&content)?;
        storyboard.validate()?;
        Ok(storyboard)
    }

    /// Save storyboard to a JSON file
    pub fn to_file(&self, path: &str) -> Result<(), crate::SdkError> {
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Validate the storyboard
    pub fn validate(&self) -> Result<(), crate::SdkError> {
        if self.title.is_empty() {
            return Err(crate::SdkError::Validation("Title is required".to_string()));
        }
        if self.duration <= 0.0 {
            return Err(crate::SdkError::Validation("Duration must be positive".to_string()));
        }
        Ok(())
    }

    /// Generate JSON schema
    pub fn json_schema() -> schemars::Schema {
        schemars::schema_for!(Storyboard)
    }
}
