//! Window configuration and management.
//!
//! This module provides utilities for creating and configuring
//! application windows with consistent defaults.

use gpui::{px, size, App, Bounds, WindowBounds, WindowOptions};

/// Default window dimensions in pixels.
pub const DEFAULT_WINDOW_SIZE: f32 = 500.0;

/// Window mode options.
#[derive(Clone, Copy, Default, PartialEq)]
pub enum WindowMode {
    /// Normal windowed mode with specified dimensions
    #[default]
    Windowed,
    /// Fullscreen mode
    Fullscreen,
    /// Maximized window mode
    Maximized,
}

/// Configuration for creating application windows.
#[derive(Clone)]
pub struct WindowConfig {
    /// Window width in pixels (used in windowed mode)
    pub width: f32,
    /// Window height in pixels (used in windowed mode)
    pub height: f32,
    /// Whether the window should be centered on screen
    pub centered: bool,
    /// Window title (note: GPUI may not use this on all platforms)
    pub title: String,
    /// Window mode (windowed, fullscreen, or maximized)
    pub mode: WindowMode,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            width: DEFAULT_WINDOW_SIZE,
            height: DEFAULT_WINDOW_SIZE,
            centered: true,
            title: "GPUI App".to_string(),
            mode: WindowMode::Windowed,
        }
    }
}

impl WindowConfig {
    /// Creates a new WindowConfig with default values (fullscreen).
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a windowed configuration with the specified size.
    pub fn windowed(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            mode: WindowMode::Windowed,
            ..Default::default()
        }
    }

    /// Creates a fullscreen configuration.
    pub fn fullscreen() -> Self {
        Self {
            mode: WindowMode::Fullscreen,
            ..Default::default()
        }
    }

    /// Sets the window dimensions (applies to windowed mode).
    ///
    /// # Arguments
    /// * `width` - Window width in pixels
    /// * `height` - Window height in pixels
    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Sets whether the window should be centered (applies to windowed mode).
    pub fn with_centered(mut self, centered: bool) -> Self {
        self.centered = centered;
        self
    }

    /// Sets the window title.
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Sets the window mode.
    pub fn with_mode(mut self, mode: WindowMode) -> Self {
        self.mode = mode;
        self
    }

    /// Sets the window to fullscreen mode.
    pub fn with_fullscreen(mut self) -> Self {
        self.mode = WindowMode::Fullscreen;
        self
    }

    /// Sets the window to windowed mode.
    pub fn with_windowed(mut self) -> Self {
        self.mode = WindowMode::Windowed;
        self
    }

    /// Creates window bounds based on the configuration.
    ///
    /// # Arguments
    /// * `cx` - The GPUI App context
    pub fn create_bounds(&self, cx: &App) -> Bounds<gpui::Pixels> {
        if self.centered {
            Bounds::centered(None, size(px(self.width), px(self.height)), cx)
        } else {
            Bounds::new(
                gpui::Point::default(),
                size(px(self.width), px(self.height)),
            )
        }
    }

    /// Creates WindowOptions based on the configuration.
    ///
    /// # Arguments
    /// * `cx` - The GPUI App context
    pub fn create_options(&self, cx: &App) -> WindowOptions {
        let window_bounds = match self.mode {
            WindowMode::Fullscreen => {
                // For fullscreen, use the entire display bounds
                let displays = cx.displays();
                if !displays.is_empty() {
                    Some(WindowBounds::Fullscreen(displays[0].bounds()))
                } else {
                    // Fallback to a reasonable fullscreen size if no displays found
                    Some(WindowBounds::Fullscreen(Bounds::new(gpui::Point::default(), size(px(1920.0), px(1080.0)))))
                }
            }
            WindowMode::Maximized => {
                // For maximized, create bounds that will be maximized by the OS
                let bounds = Bounds::centered(None, size(px(1200.0), px(800.0)), cx);
                Some(WindowBounds::Maximized(bounds))
            }
            WindowMode::Windowed => {
                let bounds = self.create_bounds(cx);
                Some(WindowBounds::Windowed(bounds))
            }
        };

        WindowOptions {
            window_bounds,
            ..Default::default()
        }
    }
}