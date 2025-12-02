//! Color swatch component for displaying color samples.
//!
//! This component displays a small colored square, useful for
//! color palettes, theme previews, or visual indicators.

use gpui::{div, Hsla, IntoElement, ParentElement, Styled};

/// Creates a color swatch element with the specified color.
///
/// # Arguments
/// * `color` - The color to display in the swatch
///
/// # Example
/// ```
/// use crate::components::color_swatch;
/// use gpui::red;
///
/// let swatch = color_swatch(red());
/// ```
pub fn color_swatch(color: Hsla) -> impl IntoElement {
    div().size_8().bg(color)
}

/// Creates a color swatch with a custom size.
///
/// # Arguments
/// * `color` - The color to display
/// * `size` - The size in pixels (applied to both width and height)
///
/// # Example
/// ```
/// use crate::components::color_swatch_sized;
/// use gpui::{red, px};
///
/// let large_swatch = color_swatch_sized(red(), px(16.0));
/// ```
pub fn color_swatch_sized(color: Hsla, size: gpui::Pixels) -> impl IntoElement {
    div().size(size).bg(color)
}

/// A row of color swatches for displaying multiple colors.
///
/// # Arguments
/// * `colors` - A vector of colors to display
///
/// # Example
/// ```
/// use crate::components::color_swatch_row;
/// use gpui::{red, green, blue};
///
/// let row = color_swatch_row(vec![red(), green(), blue()]);
/// ```
pub fn color_swatch_row(colors: Vec<Hsla>) -> impl IntoElement {
    let mut container = div().flex().gap_2();
    for color in colors {
        container = container.child(color_swatch(color));
    }
    container
}