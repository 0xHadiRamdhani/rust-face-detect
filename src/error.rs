//! Unified error handling for the face detection service.
//!
//! This module provides comprehensive error types using the `snafu` crate,
//! ensuring consistent error handling throughout the application.
//!
//! # Error Types
//!
//! The main error type [`FaceDetectionError`] covers all possible error scenarios
//! in the application, from file validation to image processing failures.
//!
//! # Context Types
//!
//! Context types like [`IoSnafu`] and [`ImageProcessingSnafu`] provide
//! convenient ways to convert external errors into our domain errors.
//!
//! # Examples
//!
//! ```rust
//! use crate::error::{FaceDetectionError, Result};
//! use snafu::ResultExt;
//!
//! fn process_image(path: &str) -> Result<()> {
//!     let img = image::open(path)
//!         .context(crate::error::ImageProcessingSnafu)?;
//!     // Process image...
//!     Ok(())
//! }
//! ```

use snafu::prelude::*;
use std::path::PathBuf;

/// Main error type for the face detection service.
/// Module for snafu context variants
pub mod context {
    use super::FaceDetectionError;
    
    /// IO operation context for snafu
    pub struct Io;
    
    /// Image processing context for snafu
    pub struct ImageProcessing;
}

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum FaceDetectionError {
    /// Invalid file format provided.
    #[snafu(display("Invalid file format: {format}"))]
    InvalidFileFormat {
        /// The file format that was rejected.
        format: String,
    },

    /// File size exceeds the maximum allowed size.
    #[snafu(display("File too large: {size} bytes (max: {max_size} bytes)"))]
    FileTooLarge {
        /// Actual file size in bytes.
        size: usize,
        /// Maximum allowed file size in bytes.
        max_size: usize,
    },

    /// No file was uploaded in the request.
    #[snafu(display("No file uploaded"))]
    NoFileUploaded,

    /// Image processing failed.
    #[snafu(display("Image processing failed"))]
    ImageProcessing {
        /// The underlying error that caused the processing failure.
        source: image::ImageError,
    },

    /// Face detection operation failed.
    #[snafu(display("Face detection failed"))]
    DetectionFailed,

    /// Internal server error occurred.
    #[snafu(display("Internal server error"))]
    InternalError,

    /// IO operation failed.
    #[snafu(display("IO error"))]
    Io {
        /// The underlying IO error.
        source: std::io::Error,
    },

    /// Multipart form parsing failed.
    #[snafu(display("Multipart parsing failed"))]
    MultipartError,

    /// Base64 encoding/decoding failed.
    #[snafu(display("Base64 error"))]
    Base64Error,

    /// Invalid image data provided.
    #[snafu(display("Invalid image data"))]
    InvalidImageData,

    /// Configuration error.
    #[snafu(display("Configuration error: {message}"))]
    Configuration {
        /// Error message describing the configuration issue.
        message: String,
    },

    /// Request validation failed.
    #[snafu(display("Request validation failed: {message}"))]
    Validation {
        /// Error message describing the validation failure.
        message: String,
    },
}

/// IO operation context for snafu
pub struct IoSnafu;

/// Image processing context for snafu
pub struct ImageProcessingSnafu;

impl snafu::IntoError<FaceDetectionError> for IoSnafu {
    type Source = std::io::Error;
    
    fn into_error(self, source: Self::Source) -> FaceDetectionError {
        FaceDetectionError::Io { source }
    }
}

impl snafu::IntoError<FaceDetectionError> for ImageProcessingSnafu {
    type Source = image::ImageError;
    
    fn into_error(self, source: Self::Source) -> FaceDetectionError {
        FaceDetectionError::ImageProcessing { source }
    }
}
    /// Invalid file format provided.
    #[snafu(display("Invalid file format: {format}"))]
    InvalidFileFormat {
        /// The file format that was rejected.
        format: String,
    },

    /// File size exceeds the maximum allowed size.
    #[snafu(display("File too large: {size} bytes (max: {max_size} bytes)"))]
    FileTooLarge {
        /// Actual file size in bytes.
        size: usize,
        /// Maximum allowed file size in bytes.
        max_size: usize,
    },

    /// No file was uploaded in the request.
    #[snafu(display("No file uploaded"))]
    NoFileUploaded,

    /// Image processing failed.
    #[snafu(display("Image processing failed: {source}"))]
    ImageProcessing {
        /// The underlying error that caused the processing failure.
        source: image::ImageError,
    },

    /// Face detection operation failed.
    #[snafu(display("Face detection failed"))]
    DetectionFailed,

    /// Internal server error occurred.
    #[snafu(display("Internal server error"))]
    InternalError,

    /// IO operation failed.
    #[snafu(display("IO error: {source}"))]
    Io {
        /// The underlying IO error.
        source: std::io::Error,
    },

    /// Multipart form parsing failed.
    #[snafu(display("Multipart parsing failed"))]
    MultipartError,

    /// Base64 encoding/decoding failed.
    #[snafu(display("Base64 error"))]
    Base64Error,

    /// Invalid image data provided.
    #[snafu(display("Invalid image data"))]
    InvalidImageData,

    /// Configuration error.
    #[snafu(display("Configuration error: {message}"))]
    Configuration {
        /// Error message describing the configuration issue.
        message: String,
    },

    /// Request validation failed.
    #[snafu(display("Request validation failed: {message}"))]
    Validation {
        /// Error message describing the validation failure.
        message: String,
    },
}

/// Type alias for results that can return `FaceDetectionError`.
pub type Result<T, E = FaceDetectionError> = std::result::Result<T, E>;

/// Convert `FaceDetectionError` to Actix-web HTTP response.
impl actix_web::error::ResponseError for FaceDetectionError {
    fn error_response(&self) -> actix_web::HttpResponse {
        use actix_web::http::StatusCode;
        
        let (status, message) = match self {
            Self::InvalidFileFormat { .. } => (StatusCode::BAD_REQUEST, "Invalid file format"),
            Self::FileTooLarge { .. } => (StatusCode::PAYLOAD_TOO_LARGE, "File too large"),
            Self::NoFileUploaded => (StatusCode::BAD_REQUEST, "No file uploaded"),
            Self::ImageProcessing { .. } => (StatusCode::BAD_REQUEST, "Invalid image format"),
            Self::DetectionFailed => (StatusCode::INTERNAL_SERVER_ERROR, "Face detection failed"),
            Self::InternalError => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
            Self::Io { .. } => (StatusCode::INTERNAL_SERVER_ERROR, "File system error"),
            Self::MultipartError => (StatusCode::BAD_REQUEST, "Invalid form data"),
            Self::Base64Error => (StatusCode::BAD_REQUEST, "Invalid image encoding"),
            Self::InvalidImageData => (StatusCode::BAD_REQUEST, "Invalid image data"),
            Self::Configuration { .. } => (StatusCode::INTERNAL_SERVER_ERROR, "Configuration error"),
            Self::Validation { .. } => (StatusCode::BAD_REQUEST, "Validation failed"),
        };

        actix_web::HttpResponse::build(status).json(serde_json::json!({
            "success": false,
            "error": message,
            "details": self.to_string()
        }))
    }
}

/// Extension trait for converting common error types to our domain errors.
pub trait IntoFaceDetectionError<T> {
    /// Convert the error to a `FaceDetectionError`.
    fn into_face_detection_error(self) -> Result<T>;
}

impl<T> IntoFaceDetectionError<T> for image::ImageError {
    fn into_face_detection_error(self) -> Result<T> {
        Err(FaceDetectionError::ImageProcessing { source: self })
    }
}

impl<T> IntoFaceDetectionError<T> for std::io::Error {
    fn into_face_detection_error(self) -> Result<T> {
        Err(FaceDetectionError::Io { source: self })
    }
}

/// Helper function to create validation errors.
pub fn validation_error(message: impl Into<String>) -> FaceDetectionError {
    FaceDetectionError::Validation {
        message: message.into(),
    }
}

/// Helper function to create configuration errors.
pub fn config_error(message: impl Into<String>) -> FaceDetectionError {
    FaceDetectionError::Configuration {
        message: message.into(),
    }
}