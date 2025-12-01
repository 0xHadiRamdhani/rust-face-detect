use actix_web::{test, web, App};
use actix_multipart::Multipart;
use std::path::Path;

#[actix_web::test]
async fn test_health_endpoint() {
    let app = test::init_service(
        App::new()
            .service(actix_web::web::resource("/api/health")
                .route(actix_web::web::get().to(face_detect_rust::api::health_check)))
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

#[actix_web::test]
async fn test_upload_endpoint_invalid_file() {
    // This test would require more complex setup with multipart forms
    // For now, we'll test the basic structure
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(
                face_detect_rust::detection::FaceDetector::new().unwrap()
            ))
            .service(face_detect_rust::api::upload_image)
    ).await;

    // Test with empty multipart form
    let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
    let body = format!(
        "------WebKitFormBoundary7MA4YWxkTrZu0gW\r\n\
         Content-Disposition: form-data; name=\"image\"; filename=\"\"\r\n\
         Content-Type: image/jpeg\r\n\r\n\
         \r\n\
         ------WebKitFormBoundary7MA4YWxkTrZu0gW--\r\n"
    );

    let req = test::TestRequest::post()
        .uri("/api/upload")
        .insert_header(("Content-Type", format!("multipart/form-data; boundary={}", boundary)))
        .set_payload(body.into_bytes())
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    // Should return error for empty file
    assert_eq!(resp.status(), 400);
}

#[actix_web::test]
async fn test_crop_endpoint_basic() {
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(
                face_detect_rust::detection::FaceDetector::new().unwrap()
            ))
            .service(face_detect_rust::api::crop_faces)
    ).await;

    // Test with valid JSON structure
    let crop_request = serde_json::json!({
        "image_data": "data:image/jpeg;base64,/9j/4AAQSkZJRgABAQAAAQABAAD/2wBDAAYEBQYFBAYGBQYHBwYIChAKCgkJChQODwwQFxQYGBcUFhYaHSUfGhsjHBYWICwgIyYnKSopGR8tMC0oMCUoKSj/2wBDAQcHBwoIChMKChMoGhYaKCgoKCgoKCgoKCgoKCgoKCgoKCgoKCgoKCgoKCgoKCgoKCgoKCgoKCgoKCgoKCgoKCj/wAARCAABAAEDASIAAhEBAxEB/8QAFQABAQAAAAAAAAAAAAAAAAAAAAv/xAAUEAEAAAAAAAAAAAAAAAAAAAAA/8QAFQEBAQAAAAAAAAAAAAAAAAAAAAX/xAAUEQEAAAAAAAAAAAAAAAAAAAAA/9oADAMBAAIRAxEAPwCwAA8A/9k=",
        "faces": [
            {
                "x": 50,
                "y": 50,
                "width": 100,
                "height": 100,
                "confidence": 0.9
            }
        ]
    });

    let req = test::TestRequest::post()
        .uri("/api/crop")
        .insert_header(("Content-Type", "application/json"))
        .set_json(&crop_request)
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["success"], true);
    assert!(body["data"]["cropped_faces"].is_array());
}

#[actix_web::test]
async fn test_crop_endpoint_invalid_data() {
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(
                face_detect_rust::detection::FaceDetector::new().unwrap()
            ))
            .service(face_detect_rust::api::crop_faces)
    ).await;

    // Test with invalid base64 data
    let crop_request = serde_json::json!({
        "image_data": "invalid-base64-data",
        "faces": []
    });

    let req = test::TestRequest::post()
        .uri("/api/crop")
        .insert_header(("Content-Type", "application/json"))
        .set_json(&crop_request)
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    // Should return error for invalid data
    assert_eq!(resp.status(), 400);
}

#[actix_web::test]
async fn test_cors_headers() {
    let app = test::init_service(
        App::new()
            .wrap(actix_cors::Cors::default()
                .allow_any_origin()
                .allow_any_method()
                .allow_any_header())
            .service(face_detect_rust::api::health_check)
    ).await;

    let req = test::TestRequest::get()
        .uri("/api/health")
        .insert_header(("Origin", "http://localhost:3000"))
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    
    // Check CORS headers
    let headers = resp.headers();
    assert!(headers.contains_key("access-control-allow-origin"));
    assert!(headers.contains_key("access-control-allow-methods"));
    assert!(headers.contains_key("access-control-allow-headers"));
}

#[actix_web::test]
async fn test_json_content_type() {
    let app = test::init_service(
        App::new()
            .service(face_detect_rust::api::health_check)
    ).await;

    let req = test::TestRequest::get()
        .uri("/api/health")
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    
    // Check that response has JSON content type
    let content_type = resp.headers().get("content-type").unwrap();
    assert!(content_type.to_str().unwrap().contains("application/json"));
}

#[actix_web::test]
async fn test_error_handling() {
    let app = test::init_service(
        App::new()
            .service(face_detect_rust::api::health_check)
    ).await;

    // Test with wrong HTTP method
    let req = test::TestRequest::post()
        .uri("/api/health")
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 405); // Method Not Allowed
}