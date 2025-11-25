# Rust CLI Echo

A simple command-line interface (CLI) program written in Rust that echoes user input back to the terminal. This project demonstrates basic Rust CLI development, including argument parsing and standard output handling.

## Features

- Accepts command-line arguments
- Echoes the provided message to stdout
- Provides usage help when no arguments are given
- Lightweight and fast Rust implementation

## Prerequisites

- [Rust](https://rustup.rs/) (install via rustup)

## Installation

1. Clone or download this repository.
2. Navigate to the project directory:
   ```bash
   cd rust-cli-echo
   ```

## Building

Build the project in debug mode:
```bash
cargo build
```

Build for release (optimized):
```bash
cargo build --release
```

## Running

Run the program directly with Cargo:
```bash
cargo run -- <your message here>
```

Or run the compiled binary:
```bash
./target/debug/rust-cli-echo <your message here>
```

For release build:
```bash
./target/release/rust-cli-echo <your message here>
```

## Usage

### Basic Usage

```bash
cargo run -- hello world
# Output: hello world
```

### Multiple Words

```bash
cargo run -- This is a test message
# Output: This is a test message
```

### Help

Run without arguments to see usage information:
```bash
cargo run
# Output:
# Usage: target/debug/rust-cli-echo <message>
# Example: target/debug/rust-cli-echo Hello World
```

## Examples

```bash
# Echo a simple message
cargo run -- Hello, Rust!

# Echo a sentence with multiple words
cargo run -- Learning Rust is fun and powerful

# Echo with special characters
cargo run -- "Hello, World! 😀"
```

## Project Structure

```
rust-cli-echo/
├── src/
│   └── main.rs          # Main application logic
├── Cargo.toml           # Project configuration
└── README.md            # This file
```

## Development

### Running Tests

This project doesn't have tests yet, but you can add them in `src/main.rs` or separate test files.

### Code Formatting

Format the code with:
```bash
cargo fmt
```

### Linting

Check for issues with:
```bash
cargo clippy
```

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- Cargo for dependency management and building