# mev-builders

[![Crates.io](https://img.shields.io/crates/v/mev-builders.svg)](https://crates.io/crates/mev-builders)
[![Documentation](https://docs.rs/mev-builders/badge.svg)](https://docs.rs/mev-builders)
[![CI](https://github.com/cakevm/mev-builders/actions/workflows/ci.yml/badge.svg)](https://github.com/cakevm/mev-builders/actions/workflows/ci.yml)
[![License](https://img.shields.io/crates/l/mev-builders)](https://github.com/cakevm/mev-builders#license)

A comprehensive, up-to-date collection of MEV builder endpoints and metadata for Ethereum, all in one place.

## Overview

`mev-builders` provides a curated list of Maximum Extractable Value (MEV) builders with their RPC endpoints, metadata, and block statistics. The data is automatically updated weekly to ensure accuracy and includes:

- Builder names and identifiers
- RPC endpoints for searchers and MEV-Share
- Block production statistics
- Signing requirements
- Special handling requirements

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
mev-builders = "0.1"
```

## Quick Start

```rust
use mev_builders::BUILDERS;

fn main() {
    // Access all builders
    for builder in BUILDERS {
        println!("{}: {}", builder.name, builder.searcher_rpc);
    }
    
    // Get the first builder's RPC URL
    println!("First builder RPC: {}", BUILDERS[0].searcher_rpc);
}
```

## Data Structure

### Builder Struct

Each builder in the collection has the following fields:

```rust
pub struct Builder<'a> {
    /// Human-readable name of the builder
    pub name: &'a str,
    
    /// Unique identifier for the builder (lowercase alphanumeric)
    pub identifier: &'a str,
    
    /// Website URL for the builder
    pub website: &'a str,
    
    /// RPC endpoint for the searcher
    pub searcher_rpc: &'a str,
    
    /// Optional RPC endpoint for MEV-Share
    pub mev_share_rpc: Option<&'a str>,
    
    /// Extra data provided by the builder in blocks
    pub extra_data: Option<&'a str>,
    
    /// Indicates if the builder requires signing for bundles
    pub signing: Signing,
    
    /// Whether an account is required to use the RPC
    pub account_required: bool,
    
    /// Number of blocks landed by this builder
    pub blocks: u64,
}
```

### Signing Requirements

```rust
pub enum Signing {
    /// Bundle gets rejected if not signed
    Required,
    
    /// Signing is optional and may give better priority
    Optional,
    
    /// Builder does not support signing
    NotSupported,
}
```

## Examples

### Filter Builders by Requirements

#### Builders Requiring Extra Handling

Some builders require special handling (custom certificates, accounts, etc.):

```rust
use mev_builders::BUILDERS;

fn main() {
    let special_builders: Vec<_> = BUILDERS
        .iter()
        .filter(|builder| builder.requires_extra_handling())
        .collect();
    
    for builder in special_builders {
        println!("{} requires special handling", builder.name);
    }
}
```

#### Active Builders (1+ Blocks)

Find builders that have successfully landed at least one block:

```rust
use mev_builders::BUILDERS;

fn main() {
    let active_builders: Vec<_> = BUILDERS
        .iter()
        .filter(|builder| builder.blocks >= 1)
        .collect();
    
    println!("Active builders: {}", active_builders.len());
    
    for builder in active_builders {
        println!("{}: {} blocks", builder.name, builder.blocks);
    }
}
```

#### Builders with MEV-Share Support

```rust
use mev_builders::BUILDERS;

fn main() {
    let mev_share_builders: Vec<_> = BUILDERS
        .iter()
        .filter(|builder| builder.mev_share_rpc.is_some())
        .collect();
    
    for builder in mev_share_builders {
        println!("{}: {}", builder.name, builder.mev_share_rpc.unwrap());
    }
}
```

### Create a Builder Map

Create a hashmap for quick lookups by identifier:

```rust
use mev_builders::{Builder, BUILDERS};
use std::collections::HashMap;

fn create_builder_map() -> HashMap<&'static str, &'static Builder<'static>> {
    BUILDERS
        .iter()
        .map(|builder| (builder.identifier, builder))
        .collect()
}

fn main() {
    let builders = create_builder_map();
    
    if let Some(flashbots) = builders.get("flashbots") {
        println!("Flashbots RPC: {}", flashbots.searcher_rpc);
    }
}
```

### Check Signing Requirements

```rust
use mev_builders::{BUILDERS, Signing};

fn main() {
    for builder in BUILDERS {
        match builder.signing {
            Signing::Required => {
                println!("{} requires bundle signing", builder.name);
            }
            Signing::Optional => {
                println!("{} supports optional signing", builder.name);
            }
            Signing::NotSupported => {
                println!("{} does not support signing", builder.name);
            }
        }
    }
}
```

## Using the Macro Directly

The `include_builders!` macro allows you to include builder data from custom JSON files:

```rust
use mev_builders_macros::include_builders;

// Include builders from custom paths
static CUSTOM_BUILDERS: &[Builder] = include_builders!(
    "path/to/builders.json",
    "path/to/builders_stats.json"
);
```

This is useful for:
- Testing with mock data
- Using private builder lists
- Creating subsets of builders

## Data Files

The builder data comes from two JSON files:

### `builders.json`

Contains the main builder information:
- Name, identifier, website
- RPC endpoints
- Signing requirements
- Account requirements
- Extra data field (used for matching with stats)

### `builders_stats.json`

Contains block production statistics:
- Key: The `extra_data` field from the builder
- Value: Number of blocks produced

Example:
```json
{
  "Titan (titanbuilder.xyz)": 22515,
  "beaverbuild.org": 12651,
  "BuilderNet": 7335
}
```

## Data Updates

The builder statistics are automatically updated weekly via GitHub Actions:
- Aggregates data from multiple sources
- Creates a pull request with updates
- Includes data consistency checks

## Contributing

Contributions are welcome! Please feel free to submit a pull request.

### Adding a New Builder

1. Add the builder information to `crates/mev-builders/data/builders.json`
2. Ensure all required fields are populated
3. Use lowercase alphanumeric for the identifier
4. Run tests to ensure data validity

### Running Tests

```bash
cargo test
```

## Acknowledgments

Data is aggregated from various sources including:
- Builder websites
- Block data
- [Flashbots dowg](https://github.com/flashbots/dowg) for MEV-Share RPC endpoints
- [Relayscan.io](https://www.relayscan.io) for builder statistics

## License
This project is licensed under the [Apache 2.0](./LICENSE-APACHE) or [MIT](./LICENSE-MIT).