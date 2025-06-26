use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
struct BuilderJson {
    name: String,
    identifier: String,
    website: String,
    searcher_rpc: String,
    mev_share_rpc: Option<String>,
    extra_data: Option<String>,
    signing: String,
    account_required: bool,
}

#[derive(Debug)]
pub struct ConsistencyReport {
    pub stats_not_in_builders: Vec<(String, u64)>,
    pub builders_not_in_stats: Vec<BuilderInfo>,
    pub builders_without_extra_data: Vec<BuilderInfo>,
}

#[derive(Debug)]
pub struct BuilderInfo {
    pub name: String,
    pub identifier: String,
    pub extra_data: Option<String>,
}

pub struct ConsistencyChecker;

impl Default for ConsistencyChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl ConsistencyChecker {
    pub fn new() -> Self {
        Self
    }

    /// Load and parse a JSON file
    fn load_json_file<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T> {
        let content = fs::read_to_string(path).context(format!("Failed to read file: {}", path.display()))?;

        serde_json::from_str(&content).context(format!("Failed to parse JSON from: {}", path.display()))
    }

    /// Check consistency between builders.json and builders_stats.json
    pub fn check_consistency(&self, builders_path: &Path, stats_path: &Path) -> Result<ConsistencyReport> {
        // Load data
        let builders: Vec<BuilderJson> = Self::load_json_file(builders_path)?;
        let stats: HashMap<String, u64> = Self::load_json_file(stats_path)?;

        // Create a map of builders by extra_data
        let mut builders_by_extra_data: HashMap<String, &BuilderJson> = HashMap::new();
        let mut builders_without_extra_data = Vec::new();

        for builder in &builders {
            if let Some(extra_data) = &builder.extra_data {
                builders_by_extra_data.insert(extra_data.clone(), builder);
            } else {
                builders_without_extra_data.push(BuilderInfo {
                    name: builder.name.clone(),
                    identifier: builder.identifier.clone(),
                    extra_data: None,
                });
            }
        }

        // Get all extra_data values from builders
        let builder_extra_data_set: HashSet<String> = builders_by_extra_data.keys().cloned().collect();

        // Get all keys from stats
        let stats_keys_set: HashSet<String> = stats.keys().cloned().collect();

        // Find stats keys not in builders
        let mut stats_not_in_builders: Vec<(String, u64)> =
            stats_keys_set.difference(&builder_extra_data_set).map(|key| (key.clone(), stats[key])).collect();
        stats_not_in_builders.sort_by(|a, b| a.0.cmp(&b.0));

        // Find builders with extra_data not in stats
        let mut builders_not_in_stats = Vec::new();
        for builder in &builders {
            if let Some(extra_data) = &builder.extra_data {
                if !stats.contains_key(extra_data) {
                    builders_not_in_stats.push(BuilderInfo {
                        name: builder.name.clone(),
                        identifier: builder.identifier.clone(),
                        extra_data: Some(extra_data.clone()),
                    });
                }
            }
        }

        Ok(ConsistencyReport { stats_not_in_builders, builders_not_in_stats, builders_without_extra_data })
    }

    /// Print the consistency report
    pub fn print_report(&self, report: &ConsistencyReport) -> bool {
        let mut has_issues = false;

        if !report.stats_not_in_builders.is_empty() {
            has_issues = true;
            println!("❌ Stats entries not found in builders.json:");
            println!("{}", "=".repeat(50));
            for (key, blocks) in &report.stats_not_in_builders {
                println!("  - '{key}' ({blocks} blocks)");
            }
            println!();
        }

        if !report.builders_not_in_stats.is_empty() {
            has_issues = true;
            println!("❌ Builders with extra_data not found in builders_stats.json:");
            println!("{}", "=".repeat(50));
            for builder in &report.builders_not_in_stats {
                println!(
                    "  - {} (identifier: {}, extra_data: '{}')",
                    builder.name,
                    builder.identifier,
                    builder.extra_data.as_deref().unwrap_or("")
                );
            }
            println!();
        }

        if !report.builders_without_extra_data.is_empty() {
            println!("⚠️  Builders without extra_data (cannot be matched with stats):");
            println!("{}", "=".repeat(50));
            for builder in &report.builders_without_extra_data {
                println!("  - {} (identifier: {})", builder.name, builder.identifier);
            }
            println!();
        }

        if !has_issues && report.builders_without_extra_data.is_empty() {
            println!("✅ All builders and stats are properly matched!");
        }

        has_issues
    }
}
