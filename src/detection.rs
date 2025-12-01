//! Face detection functionality for the service.
//! 
//! This module provides the core face detection capabilities, including
//! detection algorithms, image processing, and result formatting.

use crate::error::{FaceDetectionError, Result};
use crate::types::{DetectionResult, Face};
use image::DynamicImage;
use std::path::Path;
use std::time::Instant;

pub mod detector;
pub use detector::FaceDetector;

/// Performs face detection on an image file.
/// 
/// # Arguments
/// 
/// * `image_path` - Path to the image file
/// 
/// # Returns
/// 
/// A `DetectionResult` containing detected faces and processing information.
/// 
/// # Errors
/// 
/// Returns `FaceDetectionError` if the image cannot be processed or detection fails.
pub fn detect_faces(image_path: &Path) -> Result<DetectionResult> {
    let detector = FaceDetector::new()?;
    detector.detect_faces(image_path)
}

/// Creates a visual representation of detection results by drawing bounding boxes.
/// 
/// # Arguments
/// 
/// * `original_image` - The original image
/// * `faces` - List of detected faces
/// 
/// # Returns
/// 
/// A new image with bounding boxes drawn around detected faces.
/// 
/// # Errors
/// 
/// Returns `FaceDetectionError` if image processing fails.
pub fn visualize_detections(
    original_image: &DynamicImage,
    faces: &[Face],
) -> Result<DynamicImage> {
    let detector = FaceDetector::new()?;
    detector.draw_bounding_boxes(original_image, faces)
}

/// Crops a detected face from an image.
/// 
/// # Arguments
/// 
/// * `image` - The source image
/// * `face` - The face to crop
/// 
/// # Returns
/// 
/// A cropped image containing only the face.
/// 
/// # Errors
/// 
/// Returns `FaceDetectionError` if cropping fails or bounds are invalid.
pub fn crop_face(image: &DynamicImage, face: &Face) -> Result<DynamicImage> {
    let x = face.x.max(0) as u32;
    let y = face.y.max(0) as u32;
    let width = face.width as u32;
    let height = face.height as u32;
    
    // Ensure crop bounds are within image dimensions
    let (img_width, img_height) = image.dimensions();
    let crop_width = width.min(img_width - x);
    let crop_height = height.min(img_height - y);
    
    let cropped = image.crop_imm(x, y, crop_width, crop_height);
    Ok(cropped)
}

/// Converts an image to base64 encoded string.
/// 
/// # Arguments
/// 
/// * `image` - The image to encode
/// 
/// # Returns
/// 
/// Base64 encoded string with data URI prefix.
/// 
/// # Errors
/// 
/// Returns `FaceDetectionError` if encoding fails.
pub fn image_to_base64(image: &DynamicImage) -> Result<String> {
    use std::io::Cursor;
    
    let mut buffer = Vec::new();
    let mut cursor = Cursor::new(&mut buffer);
    
    image.write_to(&mut cursor, image::ImageOutputFormat::Jpeg(85))
        .map_err(|_| FaceDetectionError::ImageProcessing {
            source: image::ImageError::IoError(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to encode image"
            ))
        })?;
    
    Ok(format!("data:image/jpeg;base64,{}", base64_encode(&buffer)))
}

/// Decodes base64 image data.
/// 
/// # Arguments
/// 
/// * `data_uri` - Base64 encoded image with data URI prefix
/// 
/// # Returns
/// 
/// Decoded image bytes.
/// 
/// # Errors
/// 
/// Returns `FaceDetectionError` if decoding fails.
pub fn decode_base64_image(data_uri: &str) -> Result<Vec<u8>> {
    let base64_data = data_uri
        .strip_prefix("data:image/jpeg;base64,")
        .or_else(|| data_uri.strip_prefix("data:image/png;base64,"))
        .unwrap_or(data_uri);
    
    base64_decode(base64_data)
        .map_err(|_| FaceDetectionError::Base64Error)
}

/// Simple base64 encoding implementation.
fn base64_encode(data: &[u8]) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    
    let mut result = String::new();
    let mut i = 0;
    
    while i < data.len() {
        let mut buffer = [0u8; 3];
        let mut filled = 0;
        
        for j in 0..3 {
            if i + j < data.len() {
                buffer[j] = data[i + j];
                filled += 1;
            }
        }
        
        let b1 = buffer[0] >> 2;
        let b2 = ((buffer[0] & 0x03) << 4) | (buffer[1] >> 4);
        let b3 = ((buffer[1] & 0x0f) << 2) | (buffer[2] >> 6);
        let b4 = buffer[2] & 0x3f;
        
        result.push(CHARS[b1 as usize] as char);
        result.push(CHARS[b2 as usize] as char);
        
        if filled >= 2 {
            result.push(CHARS[b3 as usize] as char);
        } else {
            result.push('=');
        }
        
        if filled >= 3 {
            result.push(CHARS[b4 as usize] as char);
        } else {
            result.push('=');
        }
        
        i += 3;
    }
    
    result
}

/// Simple base64 decoding implementation.
fn base64_decode(data: &str) -> Result<Vec<u8>> {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    
    let mut result = Vec::new();
    let mut buffer = 0u32;
    let mut bits = 0;
    
    for ch in data.chars() {
        if ch == ' ' || ch == '\n' || ch == '\r' {
            continue;
        }
        
        if ch == '=' {
            break;
        }
        
        let value = CHARS.iter().position(|&c| c == ch as u8)
            .ok_or(FaceDetectionError::Base64Error)? as u32;
        
        buffer = (buffer << 6) | value;
        bits += 6;
        
        if bits >= 8 {
            bits -= 8;
            result.push((buffer >> bits) as u8);
        }
    }
    
    Ok(result)
}