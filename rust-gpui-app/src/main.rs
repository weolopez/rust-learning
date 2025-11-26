// This is the main entry point for a GPUI application in Rust.
// GPUI is a Rust GUI framework inspired by React, using a component-based architecture.
// Unlike Java's Swing or JavaScript's React, Rust's ownership system ensures memory safety without garbage collection.
//
// Key Rust concepts demonstrated:
// - Ownership and borrowing: Values have owners, and borrowing allows temporary access.
//   Unlike Java's references (which can lead to memory leaks if not careful), Rust enforces single ownership.
//   In JavaScript/Python, everything is reference-counted or GC'd, here it's compile-time checked.
// - Traits: Like interfaces in Java, but with zero-cost abstraction.
//   In Python, similar to ABCs (Abstract Base Classes).
// - No GC: Memory managed through ownership, similar to RAII in C++ but safer.
//   In Java/JavaScript/Python, GC handles this automatically at runtime cost.
// - Compared to Java/JavaScript/Python: Static typing with inference, no runtime overhead for types.
//
// In Java, you'd have a class extending a GUI component.
// In JavaScript, similar to React components with render methods.
// In Python, like Tkinter widgets, but with type safety.
//
// Imports: Bringing types and functions into scope.
// Unlike Java's import, Rust's use is for modules, traits, structs.
// prelude::* brings common items, like JavaScript's global imports.
use gpui::{div, prelude::*, px, rgb, size, App, Application, Bounds, Context, SharedString, Window, WindowBounds, WindowOptions};

// Struct definition: Like a class in Java/Python, but with no methods.
// Fields are public by default in struct, but here it's private.
// SharedString: GPUI's string type, reference-counted for sharing.
// No inheritance like Java, but composition through traits.
struct HelloWorld {
    text: SharedString,
}

// Impl block: Implementation of traits or methods for a type.
// Like Java's implements or class methods.
// Render trait: Defines how the component renders, like React's render method.
// &mut self: Mutable borrow of self, allows modifying the struct if needed.
//   Borrowing: Unlike passing 'this' in Java (which is a reference),
//   Rust's borrowing system prevents data races at compile time.
// _window: Underscore prefix means unused parameter, silences warnings.
// cx: Context, provides access to GPUI internals.
// impl IntoElement: Trait for types that can be converted to UI elements.
//   This is Rust's way of returning different types that implement IntoElement,
//   similar to Java's generic returns or Python's duck typing, but type-safe.
impl Render for HelloWorld {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        // div(): Creates a new div element, like HTML div or Java Swing panel.
        // Method chaining: Common in Rust, like builder pattern in Java.
        // .flex(), .flex_col(): CSS flexbox properties.
        // px(): Pixel units, like CSS px.
        // rgb(): Color from hex, similar to CSS colors.
        div()
            .flex()
            .flex_col()
            .gap_3()
            .bg(rgb(0x505050))
            .size(px(500.0))
            .justify_center()
            .items_center()
            .shadow_lg()
            .border_1()
            .border_color(rgb(0x0000ff))
            .text_xl()
            .text_color(rgb(0xffffff))
            // format! macro: Creates a formatted string, like String.format in Java or template literals in JS.
            // &self.text: Borrowing the text field.
            //   SharedString: GPUI's string type, optimized for sharing.
            //   Unlike Java's String (immutable), this is reference-counted internally.
            // .child(): Adds a child element to the div.
            .child(format!("Hello, {}!", &self.text))
            .child(
                div()
                    .flex()
                    .gap_2()
                    .child(div().size_8().bg(gpui::red()))
                    .child(div().size_8().bg(gpui::green()))
                    .child(div().size_8().bg(gpui::blue()))
                    .child(div().size_8().bg(gpui::yellow()))
                    .child(div().size_8().bg(gpui::black()))
                    .child(div().size_8().bg(gpui::white())),
            )
    }
}

// fn main(): Entry point, like Java's main method.
// No return type annotation needed, defaults to ().
// Unlike JavaScript/Python, main is not in a class.
fn main() {
    // Application::new(): Creates a new GPUI application.
    // .run(): Starts the event loop, takes a closure.
    // |cx: &mut App|: Closure syntax, similar to JavaScript arrow functions.
    // cx: App context, mutable borrow.
    Application::new().run(|cx: &mut App| {
        // let: Variable binding, immutable by default.
        // Bounds::centered(): Creates centered window bounds.
        // size(px(500.), px(500.0)): Size in pixels.
        // cx: Context for calculations.
        let bounds = Bounds::centered(None, size(px(500.), px(500.0)), cx);

        // cx.open_window(): Opens a new window.
        // WindowOptions: Struct with options.
        // ..Default::default(): Struct update syntax, fills rest with defaults.
        // |_, cx|: Closure for window creation.
        // cx.new(): Creates new component instance.
        // |_| HelloWorld { ... }: Closure creating HelloWorld.
        // "World".into(): Converts &str to SharedString via Into trait.
        // .unwrap(): Panics on error, like Java's exceptions but compile-time.
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| {
                cx.new(|_| HelloWorld {
                    text: "World".into(),
                })
            },
        )
        .unwrap();
    });
}
