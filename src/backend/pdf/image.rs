//! Image extraction and representation for PDF pages.

use super::types::BoundingBox;
use serde::{Deserialize, Serialize};

/// Represents an image region in a PDF page.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageRegion {
    /// Bounding box of the image on the page
    pub bbox: BoundingBox,

    /// Type classification of the image
    pub image_type: ImageType,

    /// Optional raw bitmap data (RGBA format)
    pub bitmap: Option<Vec<u8>>,

    /// Image metadata
    pub metadata: ImageMetadata,

    /// Optional confidence score for detection (0.0 to 1.0)
    pub confidence: Option<f32>,
}

impl ImageRegion {
    /// Create a new image region.
    pub fn new(bbox: BoundingBox, metadata: ImageMetadata) -> Self {
        Self {
            bbox,
            image_type: ImageType::Unknown,
            bitmap: None,
            metadata,
            confidence: None,
        }
    }

    /// Set the image type classification.
    pub fn with_type(mut self, image_type: ImageType) -> Self {
        self.image_type = image_type;
        self
    }

    /// Set the bitmap data.
    pub fn with_bitmap(mut self, bitmap: Vec<u8>) -> Self {
        self.bitmap = Some(bitmap);
        self
    }

    /// Set the confidence score.
    pub fn with_confidence(mut self, confidence: f32) -> Self {
        self.confidence = Some(confidence);
        self
    }
}

/// Classification of image types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImageType {
    /// Photographic image (complex, natural scenes)
    Photo,

    /// Diagram or schematic (simple geometric shapes)
    Diagram,

    /// Logo or icon
    Logo,

    /// Chart or data visualization
    Chart,

    /// Unknown or unclassified
    Unknown,
}

impl Default for ImageType {
    fn default() -> Self {
        Self::Unknown
    }
}

/// Metadata about an image.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageMetadata {
    /// Image width in pixels
    pub width: u32,

    /// Image height in pixels
    pub height: u32,

    /// Image format
    pub format: ImageFormat,

    /// Optional DPI (dots per inch)
    pub dpi: Option<u32>,

    /// Optional color space information
    pub color_space: Option<String>,
}

impl ImageMetadata {
    /// Create new image metadata.
    pub fn new(width: u32, height: u32, format: ImageFormat) -> Self {
        Self {
            width,
            height,
            format,
            dpi: None,
            color_space: None,
        }
    }

    /// Set DPI.
    pub fn with_dpi(mut self, dpi: u32) -> Self {
        self.dpi = Some(dpi);
        self
    }

    /// Set color space.
    pub fn with_color_space(mut self, color_space: String) -> Self {
        self.color_space = Some(color_space);
        self
    }

    /// Calculate image area in pixels.
    pub fn area(&self) -> u64 {
        self.width as u64 * self.height as u64
    }

    /// Calculate aspect ratio (width/height).
    pub fn aspect_ratio(&self) -> f64 {
        self.width as f64 / self.height as f64
    }
}

/// Image format enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImageFormat {
    /// JPEG format
    Jpeg,

    /// PNG format
    Png,

    /// BMP format
    Bmp,

    /// TIFF format
    Tiff,

    /// GIF format
    Gif,

    /// Raw bitmap data
    Raw,

    /// Unknown format
    Unknown,
}

impl Default for ImageFormat {
    fn default() -> Self {
        Self::Unknown
    }
}

impl ImageFormat {
    /// Get file extension for the format.
    pub fn extension(&self) -> &str {
        match self {
            ImageFormat::Jpeg => "jpg",
            ImageFormat::Png => "png",
            ImageFormat::Bmp => "bmp",
            ImageFormat::Tiff => "tiff",
            ImageFormat::Gif => "gif",
            ImageFormat::Raw => "raw",
            ImageFormat::Unknown => "bin",
        }
    }

    /// Get MIME type for the format.
    pub fn mime_type(&self) -> &str {
        match self {
            ImageFormat::Jpeg => "image/jpeg",
            ImageFormat::Png => "image/png",
            ImageFormat::Bmp => "image/bmp",
            ImageFormat::Tiff => "image/tiff",
            ImageFormat::Gif => "image/gif",
            ImageFormat::Raw => "application/octet-stream",
            ImageFormat::Unknown => "application/octet-stream",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_region_creation() {
        let bbox = BoundingBox::new(100.0, 200.0, 300.0, 400.0);
        let metadata = ImageMetadata::new(800, 600, ImageFormat::Jpeg);

        let region = ImageRegion::new(bbox.clone(), metadata);

        assert_eq!(region.bbox.x, 100.0);
        assert_eq!(region.image_type, ImageType::Unknown);
        assert!(region.bitmap.is_none());
        assert_eq!(region.metadata.width, 800);
    }

    #[test]
    fn test_image_region_builder() {
        let bbox = BoundingBox::new(0.0, 0.0, 100.0, 100.0);
        let metadata = ImageMetadata::new(200, 200, ImageFormat::Png);

        let region = ImageRegion::new(bbox, metadata)
            .with_type(ImageType::Photo)
            .with_confidence(0.95);

        assert_eq!(region.image_type, ImageType::Photo);
        assert_eq!(region.confidence, Some(0.95));
    }

    #[test]
    fn test_image_metadata_calculations() {
        let metadata = ImageMetadata::new(1920, 1080, ImageFormat::Jpeg);

        assert_eq!(metadata.area(), 1920 * 1080);
        assert!((metadata.aspect_ratio() - 16.0 / 9.0).abs() < 0.01);
    }

    #[test]
    fn test_image_format_properties() {
        assert_eq!(ImageFormat::Jpeg.extension(), "jpg");
        assert_eq!(ImageFormat::Png.mime_type(), "image/png");
        assert_eq!(ImageFormat::default(), ImageFormat::Unknown);
    }

    #[test]
    fn test_image_type_default() {
        assert_eq!(ImageType::default(), ImageType::Unknown);
    }

    #[test]
    fn test_metadata_with_dpi() {
        let metadata = ImageMetadata::new(300, 300, ImageFormat::Tiff).with_dpi(300);

        assert_eq!(metadata.dpi, Some(300));
    }
}
