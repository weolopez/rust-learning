//! Utility functions and helpers.
//!
//! This module provides common utility functions that can be used
//! throughout the application.

use gpui::{px, Pixels};

/// Converts a floating point value to GPUI Pixels.
///
/// # Arguments
/// * `value` - The pixel value as f32
///
/// # Example
/// ```
/// use crate::utils::to_pixels;
///
/// let size = to_pixels(100.0);
/// ```
pub fn to_pixels(value: f32) -> Pixels {
    px(value)
}

/// Clamps a value between a minimum and maximum.
///
/// # Arguments
/// * `value` - The value to clamp
/// * `min` - Minimum allowed value
/// * `max` - Maximum allowed value
///
/// # Example
/// ```
/// use crate::utils::clamp;
///
/// let clamped = clamp(150.0, 0.0, 100.0); // Returns 100.0
/// ```
pub fn clamp(value: f32, min: f32, max: f32) -> f32 {
    value.max(min).min(max)
}

/// Linearly interpolates between two values.
///
/// # Arguments
/// * `start` - Starting value
/// * `end` - Ending value
/// * `t` - Interpolation factor (0.0 to 1.0)
///
/// # Example
/// ```
/// use crate::utils::lerp;
///
/// let mid = lerp(0.0, 100.0, 0.5); // Returns 50.0
/// ```
pub fn lerp(start: f32, end: f32, t: f32) -> f32 {
    start + (end - start) * t
}