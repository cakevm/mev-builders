use anyhow::Result;
use clap::Parser;
use mev_builders::tools::consistency_checker::ConsistencyChecker;
use std::path::PathBuf;
use std::process;

#[derive(Parser, Debug)]
#[command(name = "mev-builders-check")]
#[command(about = "Check consistency between builders.json and builders_stats.json")]
#[command(version)]
struct Args {
    /// Path to builders.json file
    #[arg(short, long, default_value = "data/builders.json")]
    builders: PathBuf,

    /// Path to builders_stats.json file
    #[arg(short, long, default_value = "data/builders_stats.json")]
    stats: PathBuf,

    /// Exit with error code if inconsistencies are found
    #[arg(long)]
    fail_on_errors: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Create checker
    let checker = ConsistencyChecker::new();

    // Run consistency check
    let report = checker.check_consistency(&args.builders, &args.stats)?;

    // Print report
    let has_issues = checker.print_report(&report);

    // Exit with error if requested and issues found
    if args.fail_on_errors && has_issues {
        process::exit(1);
    }

    Ok(())
}
