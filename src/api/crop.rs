use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use crate::detection::FaceDetector;
use crate::models::{ApiError, FaceDetection};

#[derive(Debug, Serialize, Deserialize)]
pub struct CropRequest {
    pub image_data: String, // Base64 encoded image
    pub faces: Vec<FaceDetection>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CropResponse {
    pub cropped_faces: Vec<String>, // Base64 encoded cropped faces
}

#[post("/api/crop")]
pub async fn crop_faces(
    request: web::Json<CropRequest>,
    detector: web::Data<FaceDetector>,
) -> Result<HttpResponse, ApiError> {
    tracing::info!("Received crop request for {} faces", request.faces.len());
    
    // Decode base64 image
    let image_data = request.image_data
        .strip_prefix("data:image/jpeg;base64,")
        .or_else(|| request.image_data.strip_prefix("data:image/png;base64,"))
        .unwrap_or(&request.image_data);
    
    let image_bytes = base64::decode(image_data)
        .map_err(|_| ApiError::ImageProcessingFailed)?;
    
    // Load image from bytes
    let img = image::load_from_memory(&image_bytes)
        .map_err(|_| ApiError::ImageProcessingFailed)?;
    
    let mut cropped_faces = Vec::new();
    
    // Crop each face
    for (index, face) in request.faces.iter().enumerate() {
        tracing::info!("Cropping face {} at ({}, {}) size {}x{}", 
            index + 1, face.x, face.y, face.width, face.height);
        
        match crate::detection::detector::crop_face(&img, face) {
            Ok(cropped_img) => {
                // Convert to base64
                let base64_string = image_to_base64(&cropped_img)?;
                cropped_faces.push(base64_string);
            }
            Err(e) => {
                tracing::warn!("Failed to crop face {}: {}", index + 1, e);
                // Skip this face and continue with others
                continue;
            }
        }
    }
    
    tracing::info!("Successfully cropped {} faces", cropped_faces.len());
    
    let response = CropResponse {
        cropped_faces,
    };
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "data": response
    })))
}

fn image_to_base64(image: &image::DynamicImage) -> Result<String, ApiError> {
    use std::io::Cursor;
    
    let mut buffer = Vec::new();
    let mut cursor = Cursor::new(&mut buffer);
    
    image.write_to(&mut cursor, image::ImageOutputFormat::Jpeg(85))
        .map_err(|_| ApiError::ImageProcessingFailed)?;
    
    Ok(format!("data:image/jpeg;base64,{}", base64::encode(&buffer)))
}

// Simple base64 encoding implementation
mod base64 {
    use std::fmt::Write;
    
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    
    pub fn decode(data: &str) -> Result<Vec<u8>, String> {
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
            
            let value = match ch {
                'A'..='Z' => (ch as u8 - b'A') as u32,
                'a'..='z' => (ch as u8 - b'a' + 26) as u32,
                '0'..='9' => (ch as u8 - b'0' + 52) as u32,
                '+' => 62,
                '/' => 63,
                _ => return Err(format!("Invalid base64 character: {}", ch)),
            };
            
            buffer = (buffer << 6) | value;
            bits += 6;
            
            if bits >= 8 {
                bits -= 8;
                result.push((buffer >> bits) as u8);
            }
        }
        
        Ok(result)
    }
    
    pub fn encode(data: &[u8]) -> String {
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
}