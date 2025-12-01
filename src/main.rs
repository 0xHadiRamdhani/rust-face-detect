//! Face Detection Rust - Main application entry point.
//! 
//! This application provides a web service for face detection in images,
//! featuring a modern web interface and REST API.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

use actix_cors::Cors;
use actix_files;
use actix_web::{middleware, web, App, HttpServer};
use std::env;
use tracing::{error, info, Level};
use tracing_subscriber::{fmt, EnvFilter};

mod api;
mod detection;
mod error;
mod types;

use crate::api::{crop_faces, health_check, upload_image};
use crate::detector::FaceDetector;

/// Application configuration.
#[derive(Debug, Clone)]
pub struct AppConfig {
    /// Server port.
    pub port: u16,
    /// Log level.
    pub log_level: Level,
    /// Maximum file upload size in bytes.
    pub max_file_size: usize,
    /// Upload directory path.
    pub upload_dir: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            port: 8080,
            log_level: Level::INFO,
            max_file_size: 10 * 1024 * 1024, // 10MB
            upload_dir: "uploads".to_string(),
        }
    }
}

impl AppConfig {
    /// Loads configuration from environment variables.
    pub fn from_env() -> Self {
        let mut config = Self::default();
        
        // Parse port
        if let Ok(port_str) = env::var("PORT") {
            if let Ok(port) = port_str.parse::<u16>() {
                config.port = port;
            }
        }
        
        // Parse log level
        if let Ok(log_level_str) = env::var("RUST_LOG") {
            match log_level_str.to_lowercase().as_str() {
                "error" => config.log_level = Level::ERROR,
                "warn" => config.log_level = Level::WARN,
                "info" => config.log_level = Level::INFO,
                "debug" => config.log_level = Level::DEBUG,
                "trace" => config.log_level = Level::TRACE,
                _ => {}
            }
        }
        
        // Parse max file size
        if let Ok(size_str) = env::var("MAX_FILE_SIZE") {
            if let Ok(size) = size_str.parse::<usize>() {
                config.max_file_size = size;
            }
        }
        
        // Parse upload directory
        if let Ok(upload_dir) = env::var("UPLOAD_DIR") {
            config.upload_dir = upload_dir;
        }
        
        config
    }
}

/// Initializes the tracing/logging system.
fn init_tracing(config: &AppConfig) {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(config.log_level.as_str()));
    
    let subscriber = fmt()
        .with_env_filter(filter)
        .with_target(false)
        .with_thread_ids(false)
        .with_file(false)
        .with_line_number(false)
        .finish();
    
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");
}

/// Creates and configures the Actix-web application.
fn create_app(detector: web::Data<FaceDetector>, config: &AppConfig) -> App<FaceDetector> {
    App::new()
        // Add shared state
        .app_data(detector)
        .app_data(web::Data::new(config.clone()))
        
        // Configure JSON payload limits
        .app_data(web::JsonConfig::default().limit(config.max_file_size))
        .app_data(web::FormConfig::default().limit(config.max_file_size))
        
        // Enable CORS
        .wrap(
            Cors::default()
                .allow_any_origin()
                .allow_any_method()
                .allow_any_header()
                .max_age(3600)
        )
        
        // Enable request logging
        .wrap(middleware::Logger::default())
        
        // API routes
        .service(health_check)
        .service(upload_image)
        .service(crop_faces)
        
        // Static file serving
        .service(
            web::scope("/static")
                .service(actix_files::Files::new("", "./static"))
        )
        
        // Serve index.html for root
        .route("/", web::get().to(serve_index))
}

/// Serves the main HTML page.
async fn serve_index() -> actix_web::Result<actix_web::HttpResponse> {
    let html_content = include_str!("../static/index.html");
    Ok(actix_web::HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html_content))
}

/// Main application entry point.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load configuration
    let config = AppConfig::from_env();
    
    // Initialize tracing
    init_tracing(&config);
    
    info!("Starting Face Detection Rust Server v{}", env!("CARGO_PKG_VERSION"));
    info!("Configuration: {:?}", config);
    
    // Create uploads directory
    std::fs::create_dir_all(&config.upload_dir)
        .unwrap_or_else(|e| {
            error!("Failed to create uploads directory: {}", e);
            std::process::exit(1);
        });
    
    // Initialize face detector
    let detector = match FaceDetector::new() {
        Ok(detector) => {
            info!("Face detector initialized successfully");
            web::Data::new(detector)
        }
        Err(e) => {
            error!("Failed to initialize face detector: {}", e);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to initialize face detector"
            ));
        }
    };
    
    info!("Server will run on port {}", config.port);
    
    // Start HTTP server
    let server = HttpServer::new(move || create_app(detector.clone(), &config))
        .bind(("0.0.0.0", config.port))?
        .run();
    
    info!("Server started successfully at http://0.0.0.0:{}", config.port);
    
    server.await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_config_default() {
        let config = AppConfig::default();
        assert_eq!(config.port, 8080);
        assert_eq!(config.max_file_size, 10 * 1024 * 1024);
        assert_eq!(config.upload_dir, "uploads");
    }

    #[test]
    fn test_app_config_from_env() {
        std::env::set_var("PORT", "3000");
        std::env::set_var("RUST_LOG", "debug");
        
        let config = AppConfig::from_env();
        assert_eq!(config.port, 3000);
        assert_eq!(config.log_level, Level::DEBUG);
        
        std::env::remove_var("PORT");
        std::env::remove_var("RUST_LOG");
    }
}