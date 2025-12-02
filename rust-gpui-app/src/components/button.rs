//! Button component for interactive elements.
//!
//! Provides a reusable button with consistent styling and
//! support for click handlers.

use gpui::{div, Div, ParentElement, Styled};

use crate::theme::colors;

/// Creates a styled button element.
///
/// The button has:
/// - Primary background color
/// - Rounded corners
/// - Padding
/// - Text color
/// - Hover and active states (via GPUI styling)
///
/// # Example
/// ```
/// use crate::components::button;
///
/// let btn = button("Click me");
/// ```
pub fn button(label: &str) -> Div {
    div()
        .flex()
        .items_center()
        .justify_center()
        .px_4()
        .py_2()
        .bg(colors::primary())
        .text_color(colors::text())
        .rounded_md()
        .cursor_pointer()
        .child(label.to_string())
}

/// Creates a secondary styled button.
///
/// Uses the secondary color from the theme.
///
/// # Example
/// ```
/// use crate::components::button_secondary;
///
/// let btn = button_secondary("Cancel");
/// ```
pub fn button_secondary(label: &str) -> Div {
    div()
        .flex()
        .items_center()
        .justify_center()
        .px_4()
        .py_2()
        .bg(colors::secondary())
        .text_color(colors::text())
        .rounded_md()
        .cursor_pointer()
        .child(label.to_string())
}

/// Creates an outline button with transparent background.
///
/// # Example
/// ```
/// use crate::components::button_outline;
///
/// let btn = button_outline("Learn More");
/// ```
pub fn button_outline(label: &str) -> Div {
    div()
        .flex()
        .items_center()
        .justify_center()
        .px_4()
        .py_2()
        .border_1()
        .border_color(colors::primary())
        .text_color(colors::primary())
        .rounded_md()
        .cursor_pointer()
        .child(label.to_string())
}

/// Creates a small button variant.
///
/// # Example
/// ```
/// use crate::components::button_small;
///
/// let btn = button_small("OK");
/// ```
pub fn button_small(label: &str) -> Div {
    div()
        .flex()
        .items_center()
        .justify_center()
        .px_2()
        .py_1()
        .text_sm()
        .bg(colors::primary())
        .text_color(colors::text())
        .rounded_sm()
        .cursor_pointer()
        .child(label.to_string())
}