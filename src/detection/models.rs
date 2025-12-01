use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaceDetection {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub confidence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectionResult {
    pub faces: Vec<FaceDetection>,
    pub processing_time_ms: u64,
    pub total_faces: usize,
}

impl DetectionResult {
    pub fn new(faces: Vec<FaceDetection>, processing_time_ms: u64) -> Self {
        Self {
            total_faces: faces.len(),
            faces,
            processing_time_ms,
        }
    }
}