//! Face Detection Rust - A high-performance face detection web service.
//! 
//! This crate provides a complete web service for face detection in images,
//! featuring a REST API and modern web interface.
//! 
//! # Features
//! 
//! * **Face Detection**: Detect multiple faces in images with confidence scores
//! * **Image Processing**: Process and annotate images with bounding boxes
//! * **Face Cropping**: Extract individual faces from images
//! * **REST API**: Complete HTTP API for integration
//! * **Web Interface**: Modern, responsive web UI
//! * **High Performance**: Built with Rust for optimal performance
//! 
//! # Architecture
//! 
//! The service is organized into several modules:
//! 
//! * [`api`] - HTTP API endpoints
//! * [`detection`] - Core face detection functionality
//! * [`detector`] - Face detection implementation
//! * [`error`] - Unified error handling
//! * [`types`] - Type definitions and data structures
//! 
//! # Example
//! 
//! ```no_run
//! use face_detect_rust::detector::FaceDetector;
//! use std::path::Path;
//! 
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let detector = FaceDetector::new()?;
//! let result = detector.detect_faces(Path::new("image.jpg"))?;
//! println!("Detected {} faces", result.total_faces);
//! # Ok(())
//! # }
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

pub mod api;
pub mod detection;
pub mod detector;
pub mod error;
pub mod types;

// Re-export commonly used types
pub use error::{FaceDetectionError, Result};
pub use types::{ApiResponse, CropRequest, CropResponse, DetectionResult, Face, HealthResponse};
pub use detector::FaceDetector;