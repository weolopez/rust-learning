//! Home view - the main landing view of the application.
//!
//! This view displays a greeting message and a row of color swatches,
//! demonstrating the use of reusable components.

use gpui::{prelude::*, Context, SharedString, Window, div, Entity};

use crate::components::{card_full, color_swatch_row, ChatView};
use crate::theme::colors::swatch;

/// The home view component.
///
/// Displays a greeting message and color palette demo.
/// This is the refactored version of the original HelloWorld component.
pub struct HomeView {
    /// The name to greet
    pub name: SharedString,
    /// Chat view child entity
    chat_view: Entity<ChatView>,
}

impl HomeView {
    /// Creates a new HomeView with the given name.
    ///
    /// # Arguments
    /// * `cx` - The context for creating child entities
    /// * `name` - The name to display in the greeting
    ///
    /// # Example
    /// ```
    /// let view = HomeView::new(cx, "World");
    /// ```
    pub fn new(cx: &mut Context<Self>, name: impl Into<SharedString>) -> Self {
        let chat_view = cx.new(|cx| ChatView::new(cx));
        Self {
            name: name.into(),
            chat_view,
        }
    }
}

impl Render for HomeView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        // Create the color swatch row with theme colors
        let _colors = vec![
            swatch::red(),
            swatch::green(),
            swatch::blue(),
            swatch::yellow(),
            swatch::black(),
            swatch::white(),
        ];

        div()
            .flex()
            .flex_col()
            .h_full()
            // .child(
            //     card_full()
            //         .child(format!("Hello, {}!", &self.name))
            //         .child(color_swatch_row(_colors))
            // )
            .child(self.chat_view.clone())
    }
}