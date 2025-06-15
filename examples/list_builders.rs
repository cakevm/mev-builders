use mev_builders::BUILDERS;

fn main() {
    println!("Builders sorted by block count:");
    println!("================================");

    for (i, builder) in BUILDERS.iter().enumerate() {
        println!("{}. {} - {} (blocks: {})", i + 1, builder.name, builder.extra_data.unwrap_or("(no extra data)"), builder.blocks);
    }
}
