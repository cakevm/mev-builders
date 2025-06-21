# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust library that provides a comprehensive, up-to-date collection of MEV (Maximum Extractable Value) builder endpoints and metadata for Ethereum. The library uses procedural macros to compile JSON data directly into the binary at compile time, providing type-safe, zero-overhead access to builder information.

## Essential Commands

### Building
- `make build` - Build all packages in debug mode
- `make release` - Build all packages in release mode
- `cargo build --features tools` - Build with optional tools feature

### Testing
- `make test` - Run all tests
- `cargo test test_name` - Run specific test
- `cargo test -- --nocapture` - Run tests with output visible

### Code Quality
- `make fmt` - Format all code
- `make clippy` - Run clippy linter (treats warnings as errors)
- `make taplo` - Format TOML files (requires `cargo install taplo-cli`)
- `make deny-check` - Check dependencies (requires `cargo install cargo-deny`)
- `make pre-release` - Run all checks (fmt, clippy, test, taplo, deny)

### Development Workflow
Before committing changes, always run:
```bash
make pre-release
```

## Architecture & Key Concepts

### Macro-Based Data Inclusion
The core innovation is the `include_builders!()` macro in `crates/mev-builders-macros/`. This procedural macro:
1. Reads `builders.json` and `builders_stats.json` at compile time
2. Correlates builders with their statistics via the `extra_data` field
3. Generates a sorted static array `BUILDERS` embedded in the binary
4. Provides compile-time validation and type safety

### Data Model
- **Builder struct**: Contains name, identifier, RPC endpoints, signing requirements, etc.
- **Signing enum**: Required (Flashbots only), Optional, or NotSupported
- **Static lifetime**: All string data uses `&'static str` for zero-allocation access

### Weekly Updates
A GitHub Actions workflow automatically updates builder statistics weekly by:
1. Running aggregation scripts
2. Creating a pull request with updated `builders_stats.json`
3. Triggering CI checks on the PR

### Important Implementation Details

1. **Data Correlation**: Builders are matched to stats via the `extra_data` field (on-chain identifier)
2. **Sorting**: Builders are automatically sorted by block count (descending) during compilation
3. **Special Cases**: Some builders require extra handling (e.g., buildernet needs custom certs)
4. **Validation**: Comprehensive tests ensure data integrity, unique identifiers, valid URLs

### MEV Terminology
- **MEV**: Maximum Extractable Value from block production
- **Searcher RPC**: Endpoint for submitting transaction bundles
- **MEV-Share RPC**: Endpoint for MEV profit sharing protocol
- **Bundle Signing**: X-Flashbots-Signature header requirement
- **Extra Data**: On-chain identifier in block headers used to track builders

## Working with the Codebase

### Adding/Updating Builders
1. Edit `crates/mev-builders/data/builders.json`
2. Ensure all required fields are populated
3. Use lowercase alphanumeric for identifiers
4. Run `make test` to validate changes

### Using the Library
Access builder data through the static `BUILDERS` array:
```rust
use mev_builders::BUILDERS;
for builder in BUILDERS {
    // Already sorted by block count
}
```

### Tools Feature
Enable with `--features tools` to access Python script integration for data aggregation and consistency checking.

## Project Structure
- `crates/mev-builders/` - Main library crate
- `crates/mev-builders-macros/` - Procedural macro crate
- `data/` - JSON data files (builders.json, builders_stats.json)
- `examples/` - Usage examples
- `.github/workflows/` - CI/CD automation

## Requirements
- Rust 1.87 or later (uses edition 2024)
- Optional: taplo-cli for TOML formatting
- Optional: cargo-deny for dependency checking