//! Piplop API client

use reqwest::Client;

use crate::error::SdkError;
use crate::schema::Storyboard;

/// Piplop API client
pub struct PiplopClient {
    client: Client,
    base_url: String,
    api_key: Option<String>,
}

impl PiplopClient {
    /// Create a new client
    pub fn new(base_url: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.to_string(),
            api_key: None,
        }
    }

    /// Set API key for authentication
    pub fn with_api_key(mut self, api_key: &str) -> Self {
        self.api_key = Some(api_key.to_string());
        self
    }

    /// Register a storyboard as IP on Story Protocol
    pub async fn register_storyboard(&self, storyboard: &Storyboard) -> Result<String, SdkError> {
        let url = format!("{}/api/ip/register", self.base_url);
        
        let mut req = self.client.post(&url).json(&serde_json::json!({
            "target_type": "storyboard",
            "target_id": storyboard.id,
            "title": storyboard.title,
            "description": storyboard.description,
        }));
        
        if let Some(key) = &self.api_key {
            req = req.header("Authorization", format!("Bearer {}", key));
        }
        
        let resp = req.send().await?;
        
        if !resp.status().is_success() {
            let error = resp.text().await.unwrap_or_default();
            return Err(SdkError::StoryProtocol(error));
        }
        
        let result: serde_json::Value = resp.json().await?;
        let ip_id = result["ip_id"].as_str()
            .ok_or_else(|| SdkError::StoryProtocol("No IP ID in response".to_string()))?;
        
        Ok(ip_id.to_string())
    }

    /// Get IP asset status
    pub async fn get_ip_status(&self, ip_id: &str) -> Result<serde_json::Value, SdkError> {
        let url = format!("{}/api/ip/{}", self.base_url, ip_id);
        let resp = self.client.get(&url).send().await?;
        let result: serde_json::Value = resp.json().await?;
        Ok(result)
    }
}
