# System Explorer

## Overview

System Explorer is a powerful, Rust-based system management tool that extends beyond traditional file explorer capabilities. Built with Dioxus, it provides an intuitive interface for system administration and development workflows.

## Getting Started

### Prerequisites

- Rust 1.75.0 or higher
- Cargo (Rust's package manager)
- Node.js (for development)

### Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/system-explorer.git
cd system-explorer
```

2. Install dependencies:
```bash
cargo build
```

### Running the Application

To start the application in development mode:

```bash
dx serve
```

For different platforms (experimental):
```bash
dx serve --platform desktop
```

## Project Structure

```
system-explorer/
├── src/          # Rust source code
├── js-src/       # JavaScript components
├── assets/       # Static assets
└── target/       # Build output
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [Dioxus](https://dioxuslabs.com)
- Inspired by modern system management tools
