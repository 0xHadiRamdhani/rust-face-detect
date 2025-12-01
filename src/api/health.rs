use actix_web::{get, web, HttpResponse};
use chrono::Utc;
use serde_json::json;

#[get("/api/health")]
pub async fn health_check() -> HttpResponse {
    let response = json!({
        "success": true,
        "data": {
            "status": "healthy",
            "timestamp": Utc::now().to_rfc3339(),
            "version": env!("CARGO_PKG_VERSION"),
            "service": "face-detect-rust"
        }
    });
    
    HttpResponse::Ok().json(response)
}