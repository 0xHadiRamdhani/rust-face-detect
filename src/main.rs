use actix_cors::Cors;
use actix_files;
use actix_web::{middleware, web, App, HttpServer};
use std::env;
use tracing::{info, error, Level};
use tracing_subscriber::FmtSubscriber;

mod api;
mod detection;
mod models;
mod utils;

use crate::api::{health, upload, crop};
use crate::detection::FaceDetector;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");
    
    info!("Starting Face Detection Rust Server...");
    
    // Get port from environment or use default
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("Invalid port number");
    
    // Create uploads directory
    std::fs::create_dir_all("uploads").expect("Failed to create uploads directory");
    
    info!("Server will run on port {}", port);
    
    // Initialize face detector
    let face_detector = match FaceDetector::new() {
        Ok(detector) => {
            info!("Face detector initialized successfully");
            detector
        }
        Err(e) => {
            error!("Failed to initialize face detector: {}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to initialize face detector"));
        }
    };
    
    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            // Enable CORS
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
            )
            // Enable logger middleware
            .wrap(middleware::Logger::default())
            // Configure JSON payload limit
            .app_data(web::JsonConfig::default().limit(1024 * 1024 * 10)) // 10MB
            // Configure multipart form data
            .app_data(
                web::FormConfig::default()
                    .limit(1024 * 1024 * 10) // 10MB
            )
            // Health check endpoint
            .service(health::health_check)
            // Upload endpoint
            .service(upload::upload_image)
            // Crop endpoint
            .service(crop::crop_faces)
            // Serve static files
            .service(actix_web::web::scope("/static")
                .service(actix_files::Files::new("", "./static"))
            )
            // Serve index.html for root
            .route("/", actix_web::web::get().to(serve_index))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}

async fn serve_index() -> actix_web::Result<actix_web::HttpResponse> {
    let html_content = include_str!("../static/index.html");
    Ok(actix_web::HttpResponse::Ok()
        .content_type("text/html")
        .body(html_content))
}