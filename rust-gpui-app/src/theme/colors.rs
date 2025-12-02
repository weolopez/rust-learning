//! Color palette definitions for the application theme.
//!
//! This module provides semantic color constants that can be used throughout
//! the application for consistent styling.

use gpui::{rgb, Rgba};

/// Color palette with semantic naming for application-wide use.
#[derive(Clone)]
pub struct ColorPalette {
    /// Primary brand color
    pub primary: Rgba,
    /// Secondary/accent color
    pub secondary: Rgba,
    /// Background color for the main window
    pub background: Rgba,
    /// Elevated surface color (cards, modals)
    pub surface: Rgba,
    /// Primary text color
    pub text: Rgba,
    /// Muted/secondary text color
    pub text_muted: Rgba,
    /// Border color
    pub border: Rgba,
    /// Success state color
    pub success: Rgba,
    /// Warning state color
    pub warning: Rgba,
    /// Error state color
    pub error: Rgba,
}

impl Default for ColorPalette {
    fn default() -> Self {
        Self {
            primary: rgb(0x007bff).into(),
            secondary: rgb(0x6c757d).into(),
            background: rgb(0x1a1a1a).into(),
            surface: rgb(0x505050).into(),
            text: rgb(0xffffff).into(),
            text_muted: rgb(0xaaaaaa).into(),
            border: rgb(0x0000ff).into(),
            success: rgb(0x28a745).into(),
            warning: rgb(0xffc107).into(),
            error: rgb(0xdc3545).into(),
        }
    }
}

impl ColorPalette {
    /// Creates a new color palette with default colors.
    pub fn new() -> Self {
        Self::default()
    }
}

/// Convenience functions for quick access to common colors.
/// These use the default palette.

/// Returns the primary brand color.
pub fn primary() -> Rgba {
    rgb(0x007bff).into()
}

/// Returns the secondary/accent color.
pub fn secondary() -> Rgba {
    rgb(0x6c757d).into()
}

/// Returns the main background color.
pub fn background() -> Rgba {
    rgb(0x1a1a1a).into()
}

/// Returns the surface color for elevated elements.
pub fn surface() -> Rgba {
    rgb(0x505050).into()
}

/// Returns the primary text color.
pub fn text() -> Rgba {
    rgb(0xffffff).into()
}

/// Returns the muted text color.
pub fn text_muted() -> Rgba {
    rgb(0xaaaaaa).into()
}

/// Returns the border color.
pub fn border() -> Rgba {
    rgb(0x0000ff).into()
}

/// Returns the success state color.
pub fn success() -> Rgba {
    rgb(0x28a745).into()
}

/// Returns the warning state color.
pub fn warning() -> Rgba {
    rgb(0xffc107).into()
}

/// Returns the error state color.
pub fn error() -> Rgba {
    rgb(0xdc3545).into()
}

/// Swatch colors for the color display demo.
pub mod swatch {
    use gpui::Hsla;

    /// Returns red swatch color.
    pub fn red() -> Hsla {
        gpui::red()
    }

    /// Returns green swatch color.
    pub fn green() -> Hsla {
        gpui::green()
    }

    /// Returns blue swatch color.
    pub fn blue() -> Hsla {
        gpui::blue()
    }

    /// Returns yellow swatch color.
    pub fn yellow() -> Hsla {
        gpui::yellow()
    }

    /// Returns black swatch color.
    pub fn black() -> Hsla {
        gpui::black()
    }

    /// Returns white swatch color.
    pub fn white() -> Hsla {
        gpui::white()
    }
}