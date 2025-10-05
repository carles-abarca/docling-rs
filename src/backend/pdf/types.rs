//! Common types for PDF processing.

use serde::{Deserialize, Serialize};

/// Bounding box with coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct BoundingBox {
    /// Left x coordinate.
    pub x: f64,
    /// Top y coordinate.
    pub y: f64,
    /// Width.
    pub width: f64,
    /// Height.
    pub height: f64,
}

impl BoundingBox {
    /// Create a new bounding box.
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self { x, y, width, height }
    }

    /// Get right x coordinate.
    pub fn right(&self) -> f64 {
        self.x + self.width
    }

    /// Get bottom y coordinate.
    pub fn bottom(&self) -> f64 {
        self.y + self.height
    }
}

/// Page dimensions.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PageDimensions {
    /// Width in points.
    pub width: f64,
    /// Height in points.
    pub height: f64,
}

/// Page rotation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Rotation {
    /// No rotation.
    None,
    /// 90 degrees clockwise.
    Clockwise90,
    /// 180 degrees.
    Rotate180,
    /// 270 degrees clockwise (90 counter-clockwise).
    Clockwise270,
}

impl Default for Rotation {
    fn default() -> Self {
        Self::None
    }
}

/// Font information.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FontInfo {
    /// Font name.
    pub name: String,
    /// Font size in points.
    pub size: f64,
    /// Bold flag.
    pub bold: bool,
    /// Italic flag.
    pub italic: bool,
}
