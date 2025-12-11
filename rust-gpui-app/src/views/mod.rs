//! Views module - full-screen application views.
//!
//! This module contains all the views (screens/pages) of the application.
//! Each view is a self-contained component that composes smaller components
//! to create a complete UI.
//!
//! # Available Views
//!
//! - [`HomeView`] - The main landing view
//!
//! # Adding New Views
//!
//! 1. Create a new file: `src/views/my_view.rs`
//! 2. Define your view struct implementing `Render`
//! 3. Add `pub mod my_view;` below
//! 4. Re-export: `pub use my_view::MyView;`
//!
//! # Example
//!
//! ```Rust,no_run
//! use crate::views::HomeView;
//!
//! // Example usage; constructor signature may differ in implementations.
//! let _home = HomeView::new("User");
//! ```

pub mod home;

pub use home::HomeView;