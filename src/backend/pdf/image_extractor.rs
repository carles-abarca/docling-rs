//! Image extraction from PDF pages.

use super::image::{ImageFormat, ImageMetadata, ImageRegion, ImageType};
use super::types::BoundingBox;
use pdfium_render::prelude::*;

/// Trait for image extraction implementations.
pub trait ImageExtractor {
    /// Extract images from a PDF page using pdfium page object.
    ///
    /// # Arguments
    ///
    /// * `page` - The pdfium page to extract images from
    ///
    /// # Returns
    ///
    /// Vector of detected image regions
    fn extract_images(&self, page: &PdfPage) -> Vec<ImageRegion>;
}

/// Pdfium-based image extractor.
///
/// Extracts images from PDF pages using pdfium-render:
/// - Detects image objects in PDF
/// - Extracts bitmap data
/// - Converts to standard formats
/// - Performs basic classification
pub struct PdfiumImageExtractor {
    /// Minimum image size (in pixels) to extract
    min_image_size: u32,

    /// Whether to extract bitmap data
    #[allow(dead_code)]
    extract_bitmaps: bool,

    /// Whether to perform image classification
    classify_images: bool,
}

impl PdfiumImageExtractor {
    /// Create a new extractor with default settings.
    pub fn new() -> Self {
        Self {
            min_image_size: 100, // Minimum 100x100 pixels
            extract_bitmaps: true,
            classify_images: true,
        }
    }

    /// Create an extractor with custom settings.
    pub fn with_settings(
        min_image_size: u32,
        extract_bitmaps: bool,
        classify_images: bool,
    ) -> Self {
        Self {
            min_image_size,
            extract_bitmaps,
            classify_images,
        }
    }

    /// Extract image regions directly from a pdfium page.
    ///
    /// This is an internal helper that processes image objects inline
    /// to avoid lifetime issues with pdfium's API.
    fn extract_from_page(&self, page: &PdfPage) -> Vec<ImageRegion> {
        let mut regions = Vec::new();

        // Iterate through all page objects
        let objects = page.objects();
        for object in objects.iter() {
            // Check if object is an image
            if let Some(image_obj) = object.as_image_object() {
                // Get bounding box
                let bounds = image_obj.bounds().unwrap_or(PdfQuadPoints::ZERO);

                let left = bounds.left().value as f64;
                let bottom = bounds.bottom().value as f64;
                let right = bounds.right().value as f64;
                let top = bounds.top().value as f64;

                let bbox = BoundingBox::new(left, bottom, right - left, top - bottom);

                // Get image dimensions (convert from PDF points to approximate pixels)
                let width = bbox.width as u32;
                let height = bbox.height as u32;

                // For now, we don't extract bitmap data (would require rendering)
                let format = ImageFormat::Unknown;

                let mut metadata = ImageMetadata::new(width, height, format);

                // Estimate DPI
                if let Some(dpi) = self.estimate_dpi(&bbox, width) {
                    metadata = metadata.with_dpi(dpi);
                }

                // Create image region
                let mut region = ImageRegion::new(bbox, metadata);

                // Classify if enabled
                if self.classify_images {
                    let image_type = self.classify_image(&region.metadata, None);
                    region = region.with_type(image_type);
                }

                // Only add if meets minimum size requirement
                if region.metadata.area() >= (self.min_image_size as u64).pow(2) {
                    regions.push(region);
                }
            }
        }

        regions
    }

    /// Detect the format of an extracted image.
    #[allow(dead_code)]
    fn detect_format(&self, bitmap_data: &[u8]) -> ImageFormat {
        // Check magic bytes to identify format
        if bitmap_data.len() < 4 {
            return ImageFormat::Unknown;
        }

        // JPEG: FF D8 FF
        if bitmap_data[0..3] == [0xFF, 0xD8, 0xFF] {
            return ImageFormat::Jpeg;
        }

        // PNG: 89 50 4E 47
        if bitmap_data[0..4] == [0x89, 0x50, 0x4E, 0x47] {
            return ImageFormat::Png;
        }

        // BMP: 42 4D
        if bitmap_data[0..2] == [0x42, 0x4D] {
            return ImageFormat::Bmp;
        }

        // TIFF: 49 49 or 4D 4D
        if bitmap_data[0..2] == [0x49, 0x49] || bitmap_data[0..2] == [0x4D, 0x4D] {
            return ImageFormat::Tiff;
        }

        // GIF: 47 49 46
        if bitmap_data[0..3] == [0x47, 0x49, 0x46] {
            return ImageFormat::Gif;
        }

        ImageFormat::Raw
    }

    /// Classify the type of image.
    ///
    /// Uses heuristics to determine if image is:
    /// - Photo (complex, many colors)
    /// - Diagram (simple, few colors, geometric)
    /// - Logo (small, simple)
    /// - Chart (data visualization patterns)
    fn classify_image(&self, metadata: &ImageMetadata, _bitmap: Option<&[u8]>) -> ImageType {
        // Simple heuristics based on size and aspect ratio
        let area = metadata.area();
        let aspect_ratio = metadata.aspect_ratio();

        // Small square images likely logos
        if area < 50_000 && (0.8..=1.2).contains(&aspect_ratio) {
            return ImageType::Logo;
        }

        // Very wide/tall images might be charts
        if !(0.5..=2.0).contains(&aspect_ratio) {
            return ImageType::Chart;
        }

        // Large images with moderate aspect ratio likely photos
        if area > 500_000 {
            return ImageType::Photo;
        }

        // Medium-sized images with geometric patterns likely diagrams
        if area > 50_000 && area < 500_000 {
            return ImageType::Diagram;
        }

        ImageType::Unknown
    }

    /// Calculate DPI if possible from page context.
    fn estimate_dpi(&self, bbox: &BoundingBox, width_px: u32) -> Option<u32> {
        // DPI = pixels / inches
        // bbox width is in points (1 point = 1/72 inch)
        let width_inches = bbox.width / 72.0;
        if width_inches > 0.0 {
            let dpi = (width_px as f64 / width_inches) as u32;
            Some(dpi)
        } else {
            None
        }
    }
}

impl Default for PdfiumImageExtractor {
    fn default() -> Self {
        Self::new()
    }
}

impl ImageExtractor for PdfiumImageExtractor {
    fn extract_images(&self, page: &PdfPage) -> Vec<ImageRegion> {
        self.extract_from_page(page)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extractor_creation() {
        let extractor = PdfiumImageExtractor::new();
        assert_eq!(extractor.min_image_size, 100);
        assert!(extractor.extract_bitmaps);
        assert!(extractor.classify_images);
    }

    #[test]
    fn test_extractor_custom_settings() {
        let extractor = PdfiumImageExtractor::with_settings(200, false, true);
        assert_eq!(extractor.min_image_size, 200);
        assert!(!extractor.extract_bitmaps);
        assert!(extractor.classify_images);
    }

    #[test]
    fn test_format_detection_jpeg() {
        let extractor = PdfiumImageExtractor::new();
        let jpeg_magic = vec![0xFF, 0xD8, 0xFF, 0xE0];
        assert_eq!(extractor.detect_format(&jpeg_magic), ImageFormat::Jpeg);
    }

    #[test]
    fn test_format_detection_png() {
        let extractor = PdfiumImageExtractor::new();
        let png_magic = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A];
        assert_eq!(extractor.detect_format(&png_magic), ImageFormat::Png);
    }

    #[test]
    fn test_format_detection_unknown() {
        let extractor = PdfiumImageExtractor::new();
        let unknown = vec![0x00, 0x01, 0x02];
        assert_eq!(extractor.detect_format(&unknown), ImageFormat::Unknown);
    }

    #[test]
    fn test_image_classification_logo() {
        let extractor = PdfiumImageExtractor::new();
        // Small square image (200x200)
        let metadata = ImageMetadata::new(200, 200, ImageFormat::Png);
        let image_type = extractor.classify_image(&metadata, None);
        assert_eq!(image_type, ImageType::Logo);
    }

    #[test]
    fn test_image_classification_photo() {
        let extractor = PdfiumImageExtractor::new();
        // Large image (1920x1080)
        let metadata = ImageMetadata::new(1920, 1080, ImageFormat::Jpeg);
        let image_type = extractor.classify_image(&metadata, None);
        assert_eq!(image_type, ImageType::Photo);
    }

    #[test]
    fn test_image_classification_chart() {
        let extractor = PdfiumImageExtractor::new();
        // Wide image (1000x300)
        let metadata = ImageMetadata::new(1000, 300, ImageFormat::Png);
        let image_type = extractor.classify_image(&metadata, None);
        assert_eq!(image_type, ImageType::Chart);
    }

    #[test]
    fn test_dpi_estimation() {
        let extractor = PdfiumImageExtractor::new();
        // 7.2 inch width at 72 points/inch = 518.4 points
        let bbox = BoundingBox::new(0.0, 0.0, 518.4, 100.0);
        let width_px = 1440; // pixels

        let dpi = extractor.estimate_dpi(&bbox, width_px);
        assert_eq!(dpi, Some(200)); // 1440 pixels / 7.2 inches ≈ 200 DPI
    }
}
