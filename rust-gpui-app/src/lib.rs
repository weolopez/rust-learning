//! GPUI Application Library
//!
//! This is a well-structured GPUI application demonstrating modern Rust UI patterns.
//!
//! # Architecture
//!
//! The application is organized into several modules:
//!
//! - [`app`] - Application initialization and window management
//! - [`components`] - Reusable UI components (buttons, cards, etc.)
//! - [`views`] - Full-screen views/pages
//! - [`state`] - Global application state management
//! - [`theme`] - Styling, colors, and typography
//! - [`utils`] - Utility functions and helpers
//!
//! # Quick Start
//!
//! ```rust
//! use rust_gpui_app::app;
//!
//! fn main() {
//!     app::run();
//! }
//! ```
//!
//! # Custom Configuration
//!
//! ```rust
//! use rust_gpui_app::app::{run_with_config, WindowConfig};
//!
//! fn main() {
//!     let config = WindowConfig::new()
//!         .with_size(800.0, 600.0)
//!         .with_title("My Custom App");
//!     run_with_config(config, "User");
//! }
//! ```
//!
//! # Creating New Components
//!
//! See the [`components`] module for examples of reusable components.
//!
//! # Creating New Views
//!
//! See the [`views`] module for examples of full-screen views.

pub mod app;
pub mod components;
pub mod services;
pub mod state;
pub mod theme;
pub mod utils;
pub mod views;

// Re-export commonly used items
pub use app::{run, run_with_config, WindowConfig};
pub use state::AppState;
pub use theme::{ColorPalette, Theme};
pub use views::HomeView;
