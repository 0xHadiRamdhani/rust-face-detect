use image::{DynamicImage, ImageBuffer, Rgb};
use crate::models::ApiError;

pub fn create_test_image(width: u32, height: u32) -> Result<DynamicImage, ApiError> {
    // Create a simple test image with some patterns that might be detected as faces
    let mut img = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(width, height);
    
    // Fill with a light background
    for pixel in img.pixels_mut() {
        *pixel = Rgb([240, 240, 240]);
    }
    
    // Add some rectangular regions that could be detected as faces
    let face_size = 80u32;
    let spacing = 100u32;
    
    // Face 1
    if width > spacing && height > spacing {
        for y in spacing..(spacing + face_size).min(height) {
            for x in spacing..(spacing + face_size).min(width) {
                if let Some(pixel) = img.get_pixel_mut_checked(x, y) {
                    *pixel = Rgb([200, 180, 160]); // Skin tone
                }
            }
        }
    }
    
    // Face 2
    if width > spacing * 3 && height > spacing {
        for y in spacing..(spacing + face_size).min(height) {
            for x in (spacing * 3)..(spacing * 3 + face_size).min(width) {
                if let Some(pixel) = img.get_pixel_mut_checked(x, y) {
                    *pixel = Rgb([180, 160, 140]); // Different skin tone
                }
            }
        }
    }
    
    Ok(DynamicImage::ImageRgb8(img))
}

pub fn validate_image_format(path: &str) -> Result<bool, ApiError> {
    match image::open(path) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

pub fn get_image_info(path: &str) -> Result<(u32, u32, String), ApiError> {
    let img = image::open(path)
        .map_err(|_| ApiError::ImageProcessingFailed)?;
    
    let (width, height) = img.dimensions();
    let format = match img {
        DynamicImage::ImageLuma8(_) => "grayscale",
        DynamicImage::ImageRgb8(_) => "rgb",
        DynamicImage::ImageRgba8(_) => "rgba",
        _ => "unknown",
    };
    
    Ok((width, height, format.to_string()))
}