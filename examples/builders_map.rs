use mev_builders::Builder;
use std::collections::HashMap;
use std::sync::LazyLock;

// This example demonstrates how to create a static map of builders using LazyLock.
static BUILDERS_MAP: LazyLock<HashMap<&str, &Builder>> = LazyLock::new(|| {
    let mut builders = HashMap::new();
    for builder in mev_builders::BUILDERS.iter() {
        builders.insert(builder.identifier, builder);
    }
    builders
});

fn main() {
    // Access builder by identifier
    println!("Flashbots RPC URL: {}", BUILDERS_MAP.get("flashbots").map_or("Not found", |b| b.searcher_rpc));
}
