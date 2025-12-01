use std::path::Path;
use std::fs;
use tempfile::TempDir;

#[cfg(test)]
mod tests {
    use super::*;
    use face_detect_rust::detection::FaceDetector;
    use face_detect_rust::utils::image::create_test_image;

    #[test]
    fn test_face_detector_creation() {
        let detector = FaceDetector::new();
        assert!(detector.is_ok(), "FaceDetector should be created successfully");
    }

    #[test]
    fn test_mock_detection_small_image() {
        let detector = FaceDetector::new().unwrap();
        
        // Create a small test image
        let temp_dir = TempDir::new().unwrap();
        let test_image_path = temp_dir.path().join("test_small.jpg");
        
        // Create test image
        let img = create_test_image(200, 200).unwrap();
        img.save(&test_image_path).unwrap();
        
        // Test detection
        let result = detector.detect_faces(&test_image_path);
        assert!(result.is_ok(), "Detection should succeed");
        
        let detection_result = result.unwrap();
        assert_eq!(detection_result.total_faces, 1, "Should detect 1 face in small image");
        assert!(detection_result.processing_time_ms > 0, "Processing time should be positive");
    }

    #[test]
    fn test_mock_detection_medium_image() {
        let detector = FaceDetector::new().unwrap();
        
        let temp_dir = TempDir::new().unwrap();
        let test_image_path = temp_dir.path().join("test_medium.jpg");
        
        // Create test image
        let img = create_test_image(500, 500).unwrap();
        img.save(&test_image_path).unwrap();
        
        // Test detection
        let result = detector.detect_faces(&test_image_path);
        assert!(result.is_ok(), "Detection should succeed");
        
        let detection_result = result.unwrap();
        assert_eq!(detection_result.total_faces, 2, "Should detect 2 faces in medium image");
    }

    #[test]
    fn test_mock_detection_large_image() {
        let detector = FaceDetector::new().unwrap();
        
        let temp_dir = TempDir::new().unwrap();
        let test_image_path = temp_dir.path().join("test_large.jpg");
        
        // Create test image
        let img = create_test_image(700, 700).unwrap();
        img.save(&test_image_path).unwrap();
        
        // Test detection
        let result = detector.detect_faces(&test_image_path);
        assert!(result.is_ok(), "Detection should succeed");
        
        let detection_result = result.unwrap();
        assert_eq!(detection_result.total_faces, 3, "Should detect 3 faces in large image");
    }

    #[test]
    fn test_bounding_box_drawing() {
        let detector = FaceDetector::new().unwrap();
        
        // Create test image
        let img = create_test_image(300, 300).unwrap();
        
        // Create mock faces
        let faces = vec![
            face_detect_rust::detection::FaceDetection {
                x: 50,
                y: 50,
                width: 100,
                height: 100,
                confidence: 0.9,
            }
        ];
        
        // Test drawing bounding boxes
        let result = detector.draw_bounding_boxes(&img, &faces);
        assert!(result.is_ok(), "Drawing bounding boxes should succeed");
        
        let processed_image = result.unwrap();
        assert_eq!(processed_image.dimensions(), img.dimensions(), "Image dimensions should be preserved");
    }

    #[test]
    fn test_face_cropping() {
        use face_detect_rust::detection::detector::crop_face;
        
        // Create test image
        let img = create_test_image(300, 300).unwrap();
        
        // Create test face
        let face = face_detect_rust::detection::FaceDetection {
            x: 50,
            y: 50,
            width: 100,
            height: 100,
            confidence: 0.9,
        };
        
        // Test cropping
        let result = crop_face(&img, &face);
        assert!(result.is_ok(), "Face cropping should succeed");
        
        let cropped = result.unwrap();
        let (width, height) = cropped.dimensions();
        assert_eq!(width, 100, "Cropped width should match face width");
        assert_eq!(height, 100, "Cropped height should match face height");
    }

    #[test]
    fn test_invalid_image_path() {
        let detector = FaceDetector::new().unwrap();
        
        // Test with non-existent file
        let result = detector.detect_faces(Path::new("non_existent.jpg"));
        assert!(result.is_err(), "Should fail for non-existent file");
    }

    #[test]
    fn test_empty_image_detection() {
        let detector = FaceDetector::new().unwrap();
        
        let temp_dir = TempDir::new().unwrap();
        let test_image_path = temp_dir.path().join("test_empty.jpg");
        
        // Create very small image (should not detect faces)
        let img = create_test_image(50, 50).unwrap();
        img.save(&test_image_path).unwrap();
        
        // Test detection
        let result = detector.detect_faces(&test_image_path);
        assert!(result.is_ok(), "Detection should succeed");
        
        let detection_result = result.unwrap();
        assert_eq!(detection_result.total_faces, 0, "Should detect 0 faces in very small image");
    }

    #[test]
    fn test_face_detection_confidence() {
        let detector = FaceDetector::new().unwrap();
        
        let temp_dir = TempDir::new().unwrap();
        let test_image_path = temp_dir.path().join("test_confidence.jpg");
        
        // Create test image
        let img = create_test_image(400, 400).unwrap();
        img.save(&test_image_path).unwrap();
        
        // Test detection
        let result = detector.detect_faces(&test_image_path);
        assert!(result.is_ok(), "Detection should succeed");
        
        let detection_result = result.unwrap();
        
        // Check confidence values
        for face in &detection_result.faces {
            assert!(face.confidence >= 0.0 && face.confidence <= 1.0, 
                "Confidence should be between 0.0 and 1.0");
            assert!(face.confidence > 0.5, 
                "Mock confidence should be reasonably high");
        }
    }

    #[test]
    fn test_processing_time_tracking() {
        let detector = FaceDetector::new().unwrap();
        
        let temp_dir = TempDir::new().unwrap();
        let test_image_path = temp_dir.path().join("test_time.jpg");
        
        // Create test image
        let img = create_test_image(300, 300).unwrap();
        img.save(&test_image_path).unwrap();
        
        // Test detection
        let result = detector.detect_faces(&test_image_path);
        assert!(result.is_ok(), "Detection should succeed");
        
        let detection_result = result.unwrap();
        assert!(detection_result.processing_time_ms < 1000, 
            "Processing should complete within reasonable time (< 1 second)");
    }
}