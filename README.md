# wasic-components

A collection of reusable WASI (WebAssembly System Interface) components for
common functionality.

You can use [wasic](https://github.com/dineshdb/wasic), wasmtime or any runtime that supports wasi for running
these wasi components.

## Overview

This repository contains high-quality, production-ready WASM components that can
be used in various WebAssembly applications and runtimes. Each component is
designed to be:

- **Reusable**: Works across different WASI runtimes and environments
- **Secure**: Built with WASI's security model in mind
- **Efficient**: Optimized for performance and minimal resource usage
- **Well-documented**: Clear interfaces and usage examples

## Available Components

- Fetch Component, copied from wassette
- Filesystem Component, copied from wassette
- Time Component

## Building Components

### Prerequisites

- Rust with the wasm32-wasip2 target
- `cargo-component` for building WASM components
- `wasm-tools` for validation

```bash
# Install Rust wasm target
rustup target add wasm32-wasip2

# Install cargo-component
cargo install cargo-component

# Install wasm-tools
cargo install wasm-tools
```

### Building All Components

```bash
# Build all components
just build

# Or using cargo directly
cargo build --target wasm32-wasip2 --release --workspace
```

### Building Individual Components

```bash
# Build time component
just build time

# Build fetch component  
just build fetch
```

### Using Just Commands

The project includes a Justfile with convenient commands:

```bash
# Development workflow
just dev                 # Build + validate + extract WIT

# Testing and validation
just validate-wasm       # Validate built WASM components
just extract-wit         # Extract WIT interfaces
just test               # Run tests

# Code quality
just lint               # Format check + clippy
just lint-fix           # Auto-fix linting issues

# CI/CD
just ci                 # Run all CI checks
```

## Component Interfaces

### Time Component WIT

```wit
package component:time;

world time {
    /// Get the current date and time as a formatted string
    /// Returns the current timestamp in ISO 8601 format
    export get-current-time: func() -> string;
}
```

### Fetch Component WIT

```wit
package component:fetch;

world fetch {
    /// Fetch data from a URL and return the response body as a String
    export fetch: func(url: string) -> result<string, string>;
}
```

## Using Components

### In WASI Applications

1. Add the component to your project's WIT file
2. Generate bindings using `wit-bindgen`
3. Use the component functions in your application

### With Cargo Component

```bash
# Add component dependency to your cargo-component project
cargo component add --path ./pkg/time
# or
cargo component add --path ./pkg/fetch
```

### With Other Runtimes

The compiled WASM components can be used with any WASI-compliant runtime:

- Wasmtime
- WasmEdge
- Spin
- wasmedge

## Testing

```bash
# Run all tests
just test

# Run tests with verbose output
just test-verbose

# Validate WASM components
just validate-wasm

# Full CI suite
just ci
```

## Development

### Project Structure

```
wasic-components/
├── pkg/
│   ├── time/          # Time component
│   │   ├── src/
│   │   ├── wit/
│   │   └── Cargo.toml
│   └── fetch/         # Fetch component
│       ├── src/
│       ├── wit/
│       └── Cargo.toml
├── Cargo.toml         # Workspace configuration
├── Justfile          # Build commands
└── README.md         # This file
```

### Adding New Components

1. Create a new directory under `pkg/`
2. Add `Cargo.toml` with component metadata
3. Create `src/lib.rs` with component implementation
4. Define WIT interface in `wit/` directory
5. Add to workspace members in root `Cargo.toml`
6. Update Justfile with build commands if needed

### Code Quality

All components follow these standards:

- **Rust 2024 Edition**
- WASI Preview 2 compliance
- Comprehensive error handling
- Minimal dependencies
- Clear documentation
- 100% safe Rust (no `unsafe` blocks)

## Dependencies

### Workspace Dependencies

- `wit-bindgen-rt`: Runtime for WIT bindings
- `chrono`: Time handling (time component)
- `serde_json`: JSON processing (fetch component)
- `spin-sdk`: HTTP functionality (fetch component)

### Build Dependencies

- `cargo-component`: Component building
- `wasm-tools`: WASM validation and tooling
- `wit-bindgen`: WIT binding generation

## License

This project is licensed under the MIT License - see the LICENSE file for
details.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Add your component following the project structure
4. Include tests and documentation
5. Run `just ci` to ensure all checks pass
6. Submit a pull request

## Versioning

Components follow semantic versioning. All components in a workspace share the
same version number defined in the workspace `Cargo.toml`.

## Examples

See the `examples/` directory for usage examples of each component in different
contexts and runtimes.
