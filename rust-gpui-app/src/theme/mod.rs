//! Theme module for the application.
//!
//! This module provides a centralized theme system including colors,
//! spacing, and typography settings for consistent UI styling.

pub mod colors;

pub use colors::ColorPalette;

/// Spacing constants for consistent layout (in pixels).
#[derive(Clone)]
pub struct Spacing {
    /// Extra small spacing (2px)
    pub xs: f32,
    /// Small spacing (4px)
    pub sm: f32,
    /// Medium spacing (8px)
    pub md: f32,
    /// Large spacing (16px)
    pub lg: f32,
    /// Extra large spacing (24px)
    pub xl: f32,
    /// Double extra large spacing (32px)
    pub xxl: f32,
}

impl Default for Spacing {
    fn default() -> Self {
        Self {
            xs: 2.0,
            sm: 4.0,
            md: 8.0,
            lg: 16.0,
            xl: 24.0,
            xxl: 32.0,
        }
    }
}

impl Spacing {
    /// Creates new spacing with default values.
    pub fn new() -> Self {
        Self::default()
    }
}

/// Typography settings for text styling (sizes in pixels).
#[derive(Clone)]
pub struct Typography {
    /// Small text size
    pub size_sm: f32,
    /// Base text size
    pub size_base: f32,
    /// Large text size
    pub size_lg: f32,
    /// Extra large text size
    pub size_xl: f32,
    /// Heading text size
    pub size_heading: f32,
}

impl Default for Typography {
    fn default() -> Self {
        Self {
            size_sm: 12.0,
            size_base: 14.0,
            size_lg: 16.0,
            size_xl: 20.0,
            size_heading: 24.0,
        }
    }
}

impl Typography {
    /// Creates new typography with default values.
    pub fn new() -> Self {
        Self::default()
    }
}

/// Main theme struct combining all styling aspects.
#[derive(Clone)]
pub struct Theme {
    /// Color palette
    pub colors: ColorPalette,
    /// Spacing values
    pub spacing: Spacing,
    /// Typography settings
    pub typography: Typography,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            colors: ColorPalette::default(),
            spacing: Spacing::default(),
            typography: Typography::default(),
        }
    }
}

impl Theme {
    /// Creates a new theme with default values.
    pub fn new() -> Self {
        Self::default()
    }
}

/// Global theme instance for quick access.
/// In a more complex app, this would be provided through GPUI's context system.
pub fn theme() -> Theme {
    Theme::default()
}