use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Invalid file format")]
    InvalidFileFormat,
    
    #[error("File too large")]
    FileTooLarge,
    
    #[error("No file uploaded")]
    NoFileUploaded,
    
    #[error("Image processing failed")]
    ImageProcessingFailed,
    
    #[error("Face detection failed")]
    DetectionFailed,
    
    #[error("Internal server error")]
    InternalError,
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Image error: {0}")]
    ImageError(#[from] image::ImageError),
}

impl actix_web::error::ResponseError for ApiError {
    fn error_response(&self) -> actix_web::HttpResponse {
        use actix_web::http::StatusCode;
        
        let (status_code, error_message) = match self {
            ApiError::InvalidFileFormat => (StatusCode::BAD_REQUEST, "Invalid file format"),
            ApiError::FileTooLarge => (StatusCode::PAYLOAD_TOO_LARGE, "File too large"),
            ApiError::NoFileUploaded => (StatusCode::BAD_REQUEST, "No file uploaded"),
            ApiError::ImageProcessingFailed => (StatusCode::INTERNAL_SERVER_ERROR, "Image processing failed"),
            ApiError::DetectionFailed => (StatusCode::INTERNAL_SERVER_ERROR, "Face detection failed"),
            ApiError::InternalError => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
            ApiError::IoError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "File system error"),
            ApiError::ImageError(_) => (StatusCode::BAD_REQUEST, "Invalid image format"),
        };
        
        actix_web::HttpResponse::build(status_code).json(serde_json::json!({
            "success": false,
            "error": error_message,
            "details": self.to_string()
        }))
    }
}