//! Card component for containing and styling content.
//!
//! The Card component provides a styled container with consistent
//! padding, background, shadow, and border styling.

use gpui::{div, px, Div, Styled};

use crate::theme::colors;

/// Creates a styled card container.
///
/// The card has:
/// - Flex column layout
/// - Background color from theme
/// - Shadow for elevation
/// - Border styling
/// - Padding
///
/// # Example
/// ```
/// use crate::components::card;
///
/// let my_card = card()
///     .child("Card content");
/// ```
pub fn card() -> Div {
    div()
        .flex()
        .flex_col()
        .gap_3()
        .bg(colors::surface())
        .p_4()
        .shadow_lg()
        .border_1()
        .border_color(colors::border())
        .rounded_md()
}

/// Creates a card with a specific size.
///
/// # Arguments
/// * `width` - Width in pixels
/// * `height` - Height in pixels
///
/// # Example
/// ```
/// use crate::components::card_sized;
/// use gpui::px;
///
/// let sized_card = card_sized(px(300.0), px(200.0))
///     .child("Sized card content");
/// ```
pub fn card_sized(width: gpui::Pixels, height: gpui::Pixels) -> Div {
    card().w(width).h(height)
}

/// Creates a centered card that fills available space.
///
/// # Example
/// ```
/// use crate::components::card_centered;
///
/// let centered = card_centered()
///     .child("Centered content");
/// ```
pub fn card_centered() -> Div {
    card().justify_center().items_center()
}

/// Creates a full-size card (500x500 pixels) with centered content.
/// This matches the original HelloWorld component layout.
pub fn card_full() -> Div {
    card()
        .size(px(500.0))
        .justify_center()
        .items_center()
        .text_xl()
        .text_color(colors::text())
}