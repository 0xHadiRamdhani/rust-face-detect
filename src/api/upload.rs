use actix_multipart::Multipart;
use actix_web::{post, web, HttpResponse};
use futures_util::TryStreamExt;
use std::io::Write;
use std::path::Path;
use uuid::Uuid;

use crate::detection::FaceDetector;
use crate::models::{ApiError, DetectionResult};

#[post("/api/upload")]
pub async fn upload_image(
    mut payload: Multipart,
    detector: web::Data<FaceDetector>,
) -> Result<HttpResponse, ApiError> {
    tracing::info!("Received upload request");
    
    // Process multipart form data
    while let Some(mut field) = payload.try_next().await.map_err(|_| ApiError::InternalError)? {
        let content_disposition = field.content_disposition();
        
        if let Some(name) = content_disposition.get_name() {
            if name == "image" {
                // Generate unique filename
                let filename = format!("{}.jpg", Uuid::new_v4());
                let filepath = format!("uploads/{}", filename);
                
                // Create file
                let mut file = std::fs::File::create(&filepath)
                    .map_err(|_| ApiError::IoError(std::io::Error::new(std::io::ErrorKind::Other, "Failed to create file")))?;
                
                // Write field data to file
                while let Some(chunk) = field.try_next().await.map_err(|_| ApiError::InternalError)? {
                    file.write_all(&chunk)
                        .map_err(|_| ApiError::IoError(std::io::Error::new(std::io::ErrorKind::Other, "Failed to write file")))?;
                }
                
                tracing::info!("File saved: {}", filepath);
                
                // Validate file is an image
                validate_image(&filepath)?;
                
                // Perform face detection
                let detection_result = detector.detect_faces(Path::new(&filepath))?;
                
                // Load original image
                let original_image = image::open(&filepath)
                    .map_err(|e| {
                        tracing::error!("Failed to open image {}: {}", filepath, e);
                        ApiError::ImageProcessingFailed
                    })?;
                
                // Draw bounding boxes on processed image
                let processed_image = detector.draw_bounding_boxes(&original_image, &detection_result.faces)
                    .map_err(|e| {
                        tracing::error!("Failed to draw bounding boxes: {}", e);
                        e
                    })?;
                
                // Convert images to base64
                let original_base64 = image_to_base64(&original_image)
                    .map_err(|e| {
                        tracing::error!("Failed to convert original image to base64: {}", e);
                        e
                    })?;
                
                let processed_base64 = image_to_base64(&processed_image)
                    .map_err(|e| {
                        tracing::error!("Failed to convert processed image to base64: {}", e);
                        e
                    })?;
                
                // Clean up uploaded file
                if let Err(e) = std::fs::remove_file(&filepath) {
                    tracing::warn!("Failed to remove temporary file {}: {}", filepath, e);
                }
                
                // Create response
                let response_data = DetectionResult {
                    original_image: original_base64,
                    processed_image: processed_base64,
                    faces: detection_result.faces.iter().map(|f| crate::models::FaceDetection {
                        x: f.x as u32,
                        y: f.y as u32,
                        width: f.width as u32,
                        height: f.height as u32,
                        confidence: f.confidence,
                    }).collect(),
                    total_faces: detection_result.total_faces,
                    processing_time_ms: detection_result.processing_time_ms,
                };
                
                tracing::info!(
                    "Detection completed: {} faces found in {}ms", 
                    response_data.total_faces, 
                    response_data.processing_time_ms
                );
                
                return Ok(HttpResponse::Ok().json(ApiResponse::success(response_data)));
            }
        }
    }
    
    Err(ApiError::NoFileUploaded)
}

fn validate_image(filepath: &str) -> Result<(), ApiError> {
    // Try to open the image to validate it's a proper image file
    match image::open(filepath) {
        Ok(_) => {
            tracing::info!("Image validation successful for: {}", filepath);
            Ok(())
        }
        Err(e) => {
            tracing::error!("Image validation failed for {}: {}", filepath, e);
            Err(ApiError::InvalidFileFormat)
        }
    }
}

fn image_to_base64(image: &image::DynamicImage) -> Result<String, ApiError> {
    use std::io::Cursor;
    
    let mut buffer = Vec::new();
    let mut cursor = Cursor::new(&mut buffer);
    
    image.write_to(&mut cursor, image::ImageOutputFormat::Jpeg(85))
        .map_err(|_| ApiError::ImageProcessingFailed)?;
    
    Ok(format!("data:image/jpeg;base64,{}", base64::encode(&buffer)))
}

// Add base64 encoding function since it's not in the standard library
mod base64 {
    use std::fmt::Write;
    
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    
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