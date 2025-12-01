//! Type definitions for the face detection service.
//! 
//! This module contains all the data structures used throughout the application,
//! organized by domain and purpose.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Represents a detected face with its bounding box and confidence score.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Face {
    /// X coordinate of the top-left corner of the bounding box.
    pub x: u32,
    /// Y coordinate of the top-left corner of the bounding box.
    pub y: u32,
    /// Width of the bounding box.
    pub width: u32,
    /// Height of the bounding box.
    pub height: u32,
    /// Confidence score of the detection (0.0 to 1.0).
    pub confidence: f32,
}

/// Result of face detection operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DetectionResult {
    /// List of detected faces.
    pub faces: Vec<Face>,
    /// Total number of faces detected.
    pub total_faces: usize,
    /// Processing time in milliseconds.
    pub processing_time_ms: u64,
}

/// API response wrapper for consistent response format.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse<T> {
    /// Success status of the operation.
    pub success: bool,
    /// Response data if operation succeeded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    /// Error message if operation failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Response metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ResponseMetadata>,
}

/// Metadata included in API responses.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseMetadata {
    /// Timestamp when the response was generated.
    pub timestamp: DateTime<Utc>,
    /// API version.
    pub version: String,
}

/// Health check response.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HealthResponse {
    /// Service status.
    pub status: String,
    /// Timestamp of the health check.
    pub timestamp: DateTime<Utc>,
    /// Service version.
    pub version: String,
}

/// Request for face cropping operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CropRequest {
    /// Base64 encoded image data.
    pub image_data: String,
    /// List of faces to crop.
    pub faces: Vec<Face>,
}

/// Response for face cropping operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CropResponse {
    /// List of base64 encoded cropped face images.
    pub cropped_faces: Vec<String>,
}

/// Complete detection response including images.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DetectionResponse {
    /// Base64 encoded original image.
    pub original_image: String,
    /// Base64 encoded processed image with bounding boxes.
    pub processed_image: String,
    /// Detection results.
    pub detection_result: DetectionResult,
}

// Implementations
impl<T> ApiResponse<T> {
    /// Create a successful API response.
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

    /// Create an error API response.
    pub fn error(error_message: impl Into<String>) -> ApiResponse<()> {
        Self {
            success: false,
            data: None,
            error: Some(error_message.into()),
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

impl Face {
    /// Create a new face detection result.
    pub fn new(x: u32, y: u32, width: u32, height: u32, confidence: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
            confidence,
        }
    }

    /// Calculate the area of the face bounding box.
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    /// Check if the face detection has high confidence.
    pub fn is_high_confidence(&self) -> bool {
        self.confidence > 0.8
    }
}

impl DetectionResult {
    /// Create a new detection result.
    pub fn new(faces: Vec<Face>, processing_time_ms: u64) -> Self {
        Self {
            total_faces: faces.len(),
            faces,
            processing_time_ms,
        }
    }

    /// Check if any faces were detected.
    pub fn has_faces(&self) -> bool {
        !self.faces.is_empty()
    }

    /// Get faces with high confidence only.
    pub fn high_confidence_faces(&self) -> Vec<&Face> {
        self.faces.iter().filter(|f| f.is_high_confidence()).collect()
    }
}