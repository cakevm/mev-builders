use anyhow::Result;
use clap::Parser;
use mev_builders::tools::stats_aggregator::StatsAggregator;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "mev-builders-stats")]
#[command(about = "Aggregate MEV builder statistics from relayscan.io")]
#[command(version)]
struct Args {
    /// Start date in YYYY-MM-DD format (inclusive)
    #[arg(long)]
    start: Option<String>,

    /// End date in YYYY-MM-DD format (inclusive)
    #[arg(long)]
    end: Option<String>,

    /// Number of days to fetch (default: 7, ignored if start/end provided)
    #[arg(short, long, default_value = "7")]
    days: i64,

    /// Output file path
    #[arg(short, long, default_value = "data/builders_stats.json")]
    output: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Create aggregator
    let aggregator = StatsAggregator::new()?;

    // Run aggregation
    aggregator.aggregate_and_save(
        args.start.as_deref(),
        args.end.as_deref(),
        args.days,
        &args.output,
    )?;

    Ok(())
}