//! UI module
//!
//! Contains all user interface components including chat, controls, and styling.
//!
//! # Rust Traits: A Beginner's Guide
//!
//! This module demonstrates how Rust uses **traits** to share behavior between types.
//! If you're coming from object-oriented languages like Java, C++, or Python, you might
//! be familiar with **inheritance** where a child class inherits methods from a parent class.
//!
//! ## The Problem: Shared Behavior
//!
//! We have two UI panel structs: `ControlsPanel` and `ChatPanel`. Both need a way to
//! check if a mouse click is inside their bounds. In OOP, you might create a base class:
//!
//! ```text
//! // Pseudocode - NOT Rust!
//! class UIPanel {
//!     virtual bool contains_point(x, y);  // Abstract method
//! }
//! class ControlsPanel extends UIPanel { ... }
//! class ChatPanel extends UIPanel { ... }
//! ```
//!
//! ## The Rust Way: Traits
//!
//! Rust doesn't have class inheritance. Instead, it uses **traits** - which are more like
//! interfaces (Java) or protocols (Swift). A trait defines a set of methods that types can
//! implement.
//!
//! ### Key Differences from OOP Inheritance:
//!
//! | OOP Inheritance | Rust Traits |
//! |-----------------|-------------|
//! | "is-a" relationship | "can-do" relationship |
//! | Single inheritance tree | Multiple traits per type |
//! | Inherits data + behavior | Only defines behavior |
//! | Defined at class creation | Can be added to any type, even external ones |
//!
//! ### Why Traits are Powerful:
//!
//! 1. **No Diamond Problem**: Multiple traits can be implemented without ambiguity
//! 2. **Retroactive Implementation**: Add traits to types you didn't create
//! 3. **Composition over Inheritance**: Build complex behavior from simple traits
//! 4. **Static Dispatch**: Zero runtime cost when using generics
//!
//! ## How We Use It Here
//!
//! The `ContainsPoint` trait below defines a single method. Both `ControlsPanel` and
//! `ChatPanel` implement this trait with their own logic. This allows code in `state.rs`
//! to call `.contains_point(x, y)` on either type without knowing which one it is.
//!
//! ```rust,ignore
//! // In state.rs, we can use the trait uniformly:
//! let in_controls = self.controls.contains_point(mx, my);
//! let in_chat = self.chat.contains_point(mx, my);
//! ```

mod chat;
mod controls;
mod skin;

/// Represents a rectangular region on screen with position and size.
///
/// This struct stores the bounds of a UI component, making it easy to check
/// if a point (like a mouse click) is inside the region.
///
/// # For Beginners: Why This Design?
///
/// Instead of each UI panel having its own hardcoded bounds check, we store
/// the bounds as data. This follows the principle of "data-driven design" -
/// the same `contains_point` logic works for any rectangle, we just change the data.
#[derive(Clone, Copy, Debug, Default)]
pub struct Bounds {
    /// X position of the top-left corner
    pub x: f32,
    /// Y position of the top-left corner
    pub y: f32,
    /// Width of the region
    pub width: f32,
    /// Height of the region
    pub height: f32,
}

impl Bounds {
    /// Create a new bounds region
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }

    /// Check if a point (x, y) is within these bounds.
    ///
    /// This is the single, shared implementation for point containment.
    /// Any UI component that has bounds can use this same logic.
    ///
    /// # Arguments
    /// * `px` - The X coordinate to check
    /// * `py` - The Y coordinate to check
    ///
    /// # Returns
    /// `true` if the point is inside the bounds (inclusive of edges)
    pub fn contains_point(&self, px: f32, py: f32) -> bool {
        px >= self.x && px <= self.x + self.width &&
        py >= self.y && py <= self.y + self.height
    }
}

/// A trait for UI components that have rectangular bounds on screen.
///
/// This is a classic example of using traits for **polymorphism** - different types that
/// share a common interface. Any struct that implements `HasBounds` promises to provide
/// a method to get its current bounds.
///
/// # For Beginners: How Traits Enable Code Reuse
///
/// By implementing `HasBounds`, a type automatically gets the `contains_point` method
/// through a **default implementation**. This means you write the bounds-checking logic
/// once, and every UI component that implements `HasBounds` gets it for free!
///
/// # Example
///
/// ```rust,ignore
/// use rapier_physics::ui::{HasBounds, Bounds};
///
/// struct MyPanel {
///     bounds: Bounds,
/// }
///
/// impl HasBounds for MyPanel {
///     fn bounds(&self) -> Bounds {
///         self.bounds
///     }
/// }
///
/// let panel = MyPanel { bounds: Bounds::new(10.0, 10.0, 100.0, 50.0) };
/// assert!(panel.contains_point(50.0, 30.0));  // Inside!
/// assert!(!panel.contains_point(200.0, 200.0));  // Outside!
/// ```
pub trait HasBounds {
    /// Returns the current bounds of this UI component.
    ///
    /// This method should return the actual screen position and size of the component.
    /// For components that move or resize, this should return the current values.
    fn bounds(&self) -> Bounds;

    /// Check if a screen point is within this component's bounds.
    ///
    /// This is a **default implementation** - you get this method for free just by
    /// implementing `bounds()`. This is how Rust traits enable code reuse without
    /// traditional inheritance!
    ///
    /// # Arguments
    /// * `x` - The X screen coordinate (typically from `mouse_position()`)
    /// * `y` - The Y screen coordinate (typically from `mouse_position()`)
    ///
    /// # Returns
    /// `true` if the point is within bounds
    fn contains_point(&self, x: f32, y: f32) -> bool {
        self.bounds().contains_point(x, y)
    }
}

// Keep the old trait as an alias for backwards compatibility
#[deprecated(note = "Use HasBounds trait instead")]
pub trait ContainsPoint {
    fn contains_point(&self, x: f32, y: f32) -> bool;
}

pub use chat::{ChatMessage, ChatPanel, ChatCommand};
pub use controls::{ControlsPanel, ControlsResult};
pub use skin::create_custom_skin;