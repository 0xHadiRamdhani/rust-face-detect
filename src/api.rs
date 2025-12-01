//! HTTP API endpoints for the face detection service.
//! 
//! This module contains all the REST API endpoints, organized by functionality.

use actix_web::{get, post, web, HttpResponse};
use crate::error::{FaceDetectionError, Result, IoSnafu, ImageProcessingSnafu};
use crate::types::{ApiResponse, CropRequest, CropResponse, DetectionResponse, HealthResponse};
use crate::detector::FaceDetector;
use snafu::ResultExt;
use std::path::Path;

/// Health check endpoint.
/// 
/// Returns the current health status of the service.
#[get("/api/health")]
pub async fn health_check() -> HttpResponse {
    let response = HealthResponse::default();
    HttpResponse::Ok().json(ApiResponse::success(response))
}

/// Image upload and face detection endpoint.
/// 
/// Accepts an image file via multipart form data and returns detection results.
#[post("/api/upload")]
pub async fn upload_image(
    mut payload: actix_multipart::Multipart,
    detector: web::Data<FaceDetector>,
) -> Result<HttpResponse> {
    use futures_util::TryStreamExt;
    use std::io::Write;
    use uuid::Uuid;
    
    tracing::info!("Received upload request");
    
    // Process multipart form data
    while let Some(mut field) = payload.try_next().await
        .map_err(|_| FaceDetectionError::MultipartError)? {
        let content_disposition = field.content_disposition();
        
        if let Some(name) = content_disposition.get_name() {
            if name == "image" {
                // Generate unique filename
                let filename = format!("{}.jpg", Uuid::new_v4());
                let filepath = format!("uploads/{}", filename);
                
                // Create file
                let mut file = std::fs::File::create(&filepath)
                    .context(IoSnafu)?;
                
                // Write field data to file
                while let Some(chunk) = field.try_next().await
                    .map_err(|_| FaceDetectionError::MultipartError)? {
                    file.write_all(&chunk)
                        .context(IoSnafu)?;
                }
                
                tracing::info!("File saved: {}", filepath);
                
                // Validate file is an image
                validate_image_file(&filepath)?;
                
                // Perform face detection
                let detection_result = detector.detect_faces(Path::new(&filepath))?;
                
                // Load original image
                let original_image = image::open(&filepath)
                    .context(ImageProcessingSnafu)?;
                
                // Draw bounding boxes on processed image
                let processed_image = detector.draw_bounding_boxes(&original_image, &detection_result.faces)?;
                
                // Convert images to base64
                let original_base64 = crate::detection::image_to_base64(&original_image)?;
                let processed_base64 = crate::detection::image_to_base64(&processed_image)?;
                
                // Clean up uploaded file
                if let Err(e) = std::fs::remove_file(&filepath) {
                    tracing::warn!("Failed to remove temporary file {}: {}", filepath, e);
                }
                
                // Create response
                let response_data = DetectionResponse {
                    original_image: original_base64,
                    processed_image: processed_base64,
                    detection_result,
                };
                
                tracing::info!(
                    "Detection completed: {} faces found in {}ms", 
                    response_data.detection_result.total_faces, 
                    response_data.detection_result.processing_time_ms
                );
                
                return Ok(HttpResponse::Ok().json(ApiResponse::success(response_data)));
            }
        }
    }
    
    Err(FaceDetectionError::NoFileUploaded)
}

/// Face cropping endpoint.
/// 
/// Accepts an image and face coordinates, returns cropped face images.
#[post("/api/crop")]
pub async fn crop_faces(
    request: web::Json<CropRequest>,
    _detector: web::Data<FaceDetector>,
) -> Result<HttpResponse> {
    tracing::info!("Received crop request for {} faces", request.faces.len());
    
    // Decode base64 image
    let image_bytes = crate::detection::decode_base64_image(&request.image_data)?;
    
    // Load image from bytes
    let img = image::load_from_memory(&image_bytes)
        .context(ImageProcessingSnafu)?;
    
    let mut cropped_faces = Vec::new();
    
    // Crop each face
    for (index, face) in request.faces.iter().enumerate() {
        tracing::info!("Cropping face {} at ({}, {}) size {}x{}", 
            index + 1, face.x, face.y, face.width, face.height);
        
        match crate::detection::crop_face(&img, face) {
            Ok(cropped_img) => {
                // Convert to base64
                let base64_string = crate::detection::image_to_base64(&cropped_img)?;
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
    
    Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
}

/// Validates that a file is a valid image.
/// 
/// # Arguments
/// 
/// * `filepath` - Path to the file to validate
/// 
/// # Returns
/// 
/// Ok(()) if the file is a valid image, error otherwise.
fn validate_image_file(filepath: &str) -> Result<()> {
    match image::open(filepath) {
        Ok(_) => {
            tracing::info!("Image validation successful for: {}", filepath);
            Ok(())
        }
        Err(e) => {
            tracing::error!("Image validation failed for {}: {}", filepath, e);
            Err(FaceDetectionError::ImageProcessing { source: e })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, App};

    #[actix_web::test]
    async fn test_health_endpoint() {
        let app = test::init_service(
            App::new().service(health_check)
        ).await;

        let req = test::TestRequest::get()
            .uri("/api/health")
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        
        let body: serde_json::Value = test::read_body_json(resp).await;
        assert_eq!(body["success"], true);
        assert_eq!(body["data"]["status"], "healthy");
    }
}