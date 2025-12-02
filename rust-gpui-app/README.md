# Rust GPUI Learning App

This is a learning project for Rust GUI programming using the GPUI framework. The application demonstrates a well-structured, modular architecture for building scalable UI applications.

## Architecture

This application follows a modular, component-based architecture. For detailed documentation, see [ARCHITECTURE.md](./ARCHITECTURE.md).

### Project Structure

```
src/
├── main.rs              # Entry point - minimal, delegates to app
├── lib.rs               # Library root - re-exports public API
├── app/                 # Application initialization
│   ├── mod.rs           # App setup and run functions
│   └── window.rs        # Window configuration
├── components/          # Reusable UI components
│   ├── mod.rs           # Component exports
│   ├── button.rs        # Button component
│   ├── card.rs          # Card container
│   └── color_swatch.rs  # Color display
├── views/               # Full-screen views
│   ├── mod.rs           # View exports
│   └── home.rs          # Home view
├── state/               # State management
│   └── mod.rs           # AppState
├── theme/               # Styling
│   ├── mod.rs           # Theme configuration
│   └── colors.rs        # Color palette
└── utils/               # Utilities
    └── mod.rs           # Helper functions
```

### Key Concepts

- **Components**: Reusable UI building blocks in `src/components/`
- **Views**: Full-screen layouts that compose components in `src/views/`
- **Theme**: Centralized styling and colors in `src/theme/`
- **State**: Global state management in `src/state/`

## Getting Started

### Prerequisites

- Rust (latest stable version)
- On macOS: Xcode and command line tools with Metal support

### Installation

1. Install Xcode from the App Store
2. Install command line tools: `xcode-select --install`
3. Ensure Xcode is selected: `sudo xcode-select --switch /Applications/Xcode.app/Contents/Developer`

### Building and Running

```bash
cargo build
cargo run
```

### Custom Configuration

```rust
use rust_gpui_app::app::{run_with_config, WindowConfig};

fn main() {
    let config = WindowConfig::new()
        .with_size(800.0, 600.0)
        .with_title("My Custom App");
    run_with_config(config, "User");
}
```

## Extending the Application

### Adding a New Component

1. Create `src/components/my_component.rs`
2. Add `pub mod my_component;` to `src/components/mod.rs`
3. Re-export: `pub use my_component::MyComponent;`

### Adding a New View

1. Create `src/views/my_view.rs`
2. Implement the `Render` trait
3. Add to `src/views/mod.rs`

### Adding Colors

1. Add constants to `src/theme/colors.rs`
2. Use semantic naming (e.g., `surface_elevated()`)

## Resources

- [GPUI Documentation](https://docs.rs/gpui/latest/gpui/)
- [GPUI Examples](https://www.gpui.rs/)
- [Awesome GPUI](https://github.com/zed-industries/awesome-gpui)
- [GPUI Tutorial](https://github.com/hedge-ops/gpui-tutorial)
