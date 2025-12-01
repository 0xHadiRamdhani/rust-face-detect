use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub metadata: Option<ResponseMetadata>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseMetadata {
    pub timestamp: DateTime<Utc>,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FaceDetection {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub confidence: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DetectionResult {
    pub original_image: String,  // Base64 encoded
    pub processed_image: String, // Base64 encoded with bounding boxes
    pub faces: Vec<FaceDetection>,
    pub total_faces: usize,
    pub processing_time_ms: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: DateTime<Utc>,
    pub version: String,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            metadata: Some(ResponseMetadata {
                timestamp: Utc::now(),
                version: env!("CARGO_PKG_VERSION").to_string(),
            }),
        }
    }
    
    pub fn error(error_message: String) -> ApiResponse<()> {
        Self {
            success: false,
            data: None,
            error: Some(error_message),
            metadata: Some(ResponseMetadata {
                timestamp: Utc::now(),
                version: env!("CARGO_PKG_VERSION").to_string(),
            }),
        }
    }
}

impl Default for HealthResponse {
    fn default() -> Self {
        Self {
            status: "healthy".to_string(),
            timestamp: Utc::now(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}