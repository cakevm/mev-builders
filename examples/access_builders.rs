use mev_builders::BUILDERS;

fn main() {
    // Amount of builders
    println!("Number of builders: {}", BUILDERS.len());

    // Access the first builder's RPC URL
    println!("First builders rpc: {}", BUILDERS[0].searcher_rpc);
}
