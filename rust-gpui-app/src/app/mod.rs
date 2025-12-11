//! Application module - core application setup and initialization.
//!
//! This module handles the main application lifecycle including:
//! - Application creation and configuration
//! - Window management
//! - Initial state setup
//!
//! # Example
//!
//! ```
//! use rust_gpui_app::app;
//!
//! fn main() {
//!     app::run();
//! }
//! ```

pub mod window;

use gpui::{prelude::*, App, Application};

use crate::views::HomeView;
pub use window::WindowConfig;

/// Runs the GPUI application.
///
/// This is the main entry point for starting the application.
/// It creates the GPUI Application, configures the window,
/// and starts the event loop.
///
/// # Example
///
/// ```
/// use rust_gpui_app::app;
///
/// fn main() {
///     app::run();
/// }
/// ```
pub fn run() {
    Application::new().run(|cx: &mut App| {
        let config = WindowConfig::fullscreen();
        let options = config.create_options(cx);

        cx.open_window(options, |_, cx| cx.new(|cx| HomeView::new(cx, "World")))
            .unwrap();

    });
}

/// Runs the application with a custom configuration.
///
/// # Arguments
/// * `config` - Window configuration options
/// * `name` - The name to display in the greeting
///
/// # Example
///
/// ```
/// use rust_gpui_app::app::{run_with_config, WindowConfig};
///
/// fn main() {
///     let config = WindowConfig::new()
///         .with_size(800.0, 600.0)
///         .with_title("My App");
///     run_with_config(config, "User");
/// }
/// ```
pub fn run_with_config(config: WindowConfig, name: &str) {
    let name = name.to_string();
    Application::new().run(move |cx: &mut App| {
        let options = config.create_options(cx);

        cx.open_window(options, |_, cx| cx.new(|cx| HomeView::new(cx, name.clone())))
            .unwrap();
    });
}