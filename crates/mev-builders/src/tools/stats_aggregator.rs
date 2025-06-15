use anyhow::{Context, Result};
use chrono::{DateTime, Duration, Local, NaiveDate};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct RelayResponse {
    builders: Vec<RelayBuilder>,
}

#[derive(Debug, Deserialize)]
struct RelayBuilder {
    info: BuilderInfo,
    children: Option<Vec<ChildBuilder>>,
}

#[derive(Debug, Deserialize)]
struct BuilderInfo {
    extra_data: String,
    num_blocks: u64,
}

#[derive(Debug, Deserialize)]
struct ChildBuilder {
    extra_data: String,
    num_blocks: u64,
}

#[derive(Debug, Serialize)]
pub struct HierarchicalBuilder {
    pub name: String,
    pub blocks: u64,
    pub children: Vec<HierarchicalChild>,
}

#[derive(Debug, Serialize)]
pub struct HierarchicalChild {
    pub name: String,
    pub blocks: u64,
}

pub struct StatsAggregator {
    client: Client,
}

impl StatsAggregator {
    pub fn new() -> Result<Self> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .context("Failed to create HTTP client")?;
        
        Ok(Self { client })
    }

    /// Fetch relay data for a specific date
    pub fn fetch_relay_data(&self, date: &str) -> Result<RelayResponse> {
        let url = format!("https://www.relayscan.io/stats/day/{}/json", date);
        
        let response = self.client
            .get(&url)
            .send()
            .context(format!("Failed to fetch data for {}", date))?;
        
        if !response.status().is_success() {
            anyhow::bail!("HTTP error: {}", response.status());
        }
        
        response.json()
            .context("Failed to parse JSON response")
    }

    /// Aggregate builders from relay data
    pub fn aggregate_builders(&self, builders: Vec<RelayBuilder>) -> (Vec<HierarchicalBuilder>, HashMap<String, u64>) {
        let mut hierarchical = Vec::new();
        let mut flat_aggregated = HashMap::new();

        for builder in builders {
            let extra_data = builder.info.extra_data.trim().to_string();
            let num_blocks = builder.info.num_blocks;

            // Add to flat aggregation
            *flat_aggregated.entry(extra_data.clone()).or_insert(0) += num_blocks;

            // Create hierarchical structure
            let mut builder_info = HierarchicalBuilder {
                name: extra_data.clone(),
                blocks: num_blocks,
                children: Vec::new(),
            };

            // Add children if they exist
            if let Some(children) = builder.children {
                let mut child_vec = Vec::new();
                
                for child in children {
                    let child_extra_data = child.extra_data.trim().to_string();
                    let child_blocks = child.num_blocks;

                    // Add to flat aggregation
                    *flat_aggregated.entry(child_extra_data.clone()).or_insert(0) += child_blocks;

                    child_vec.push(HierarchicalChild {
                        name: child_extra_data,
                        blocks: child_blocks,
                    });
                }

                // Sort children by blocks (descending)
                child_vec.sort_by(|a, b| b.blocks.cmp(&a.blocks));
                builder_info.children = child_vec;
            }

            hierarchical.push(builder_info);
        }

        // Sort parents by blocks (descending)
        hierarchical.sort_by(|a, b| b.blocks.cmp(&a.blocks));

        (hierarchical, flat_aggregated)
    }

    /// Get date range for fetching data
    pub fn get_date_range(start: Option<&str>, end: Option<&str>, days: i64) -> Result<Vec<String>> {
        let dates = if let (Some(start_str), Some(end_str)) = (start, end) {
            let start_date = NaiveDate::parse_from_str(start_str, "%Y-%m-%d")
                .context("Invalid start date format")?;
            let end_date = NaiveDate::parse_from_str(end_str, "%Y-%m-%d")
                .context("Invalid end date format")?;

            let (start_date, end_date) = if start_date > end_date {
                (end_date, start_date)
            } else {
                (start_date, end_date)
            };

            let mut dates = Vec::new();
            let mut current = start_date;
            
            while current <= end_date {
                dates.push(current.format("%Y-%m-%d").to_string());
                current = current.succ_opt().unwrap();
            }
            
            dates
        } else {
            // Default: go back 'days' days from yesterday
            let today = Local::now();
            let yesterday = today - Duration::days(1);
            
            let mut dates = Vec::new();
            for i in 0..days {
                let date = yesterday - Duration::days(i);
                dates.push(date.format("%Y-%m-%d").to_string());
            }
            
            dates.reverse(); // Sort chronologically
            dates
        };

        Ok(dates)
    }

    /// Merge hierarchical data from multiple days
    pub fn merge_hierarchical_data(&self, all_data: Vec<Vec<HierarchicalBuilder>>) -> Vec<HierarchicalBuilder> {
        let mut merged: HashMap<String, (u64, HashMap<String, u64>)> = HashMap::new();

        // Merge all data
        for daily_data in all_data {
            for builder in daily_data {
                let entry = merged.entry(builder.name.clone()).or_insert((0, HashMap::new()));
                entry.0 += builder.blocks;

                for child in builder.children {
                    *entry.1.entry(child.name).or_insert(0) += child.blocks;
                }
            }
        }

        // Convert to final format
        let mut result: Vec<HierarchicalBuilder> = merged.into_iter()
            .map(|(name, (blocks, children_map))| {
                let mut children: Vec<HierarchicalChild> = children_map.into_iter()
                    .map(|(child_name, child_blocks)| HierarchicalChild {
                        name: child_name,
                        blocks: child_blocks,
                    })
                    .collect();

                // Sort children by blocks (descending)
                children.sort_by(|a, b| b.blocks.cmp(&a.blocks));

                HierarchicalBuilder {
                    name,
                    blocks,
                    children,
                }
            })
            .collect();

        // Sort parents by blocks (descending)
        result.sort_by(|a, b| b.blocks.cmp(&a.blocks));
        
        result
    }

    /// Aggregate stats for a date range and save to file
    pub fn aggregate_and_save(&self, start: Option<&str>, end: Option<&str>, days: i64, output_path: &Path) -> Result<()> {
        let dates = Self::get_date_range(start, end, days)?;
        
        println!("Fetching data for dates: {} to {} ({} days)", 
                 dates.first().unwrap(), 
                 dates.last().unwrap(), 
                 dates.len());
        println!("{}", "=".repeat(60));

        let mut all_hierarchical_data = Vec::new();
        let mut total_flat_aggregated: HashMap<String, u64> = HashMap::new();

        for date_str in &dates {
            println!("Fetching data for {}...", date_str);
            
            match self.fetch_relay_data(date_str) {
                Ok(data) => {
                    let (hierarchical, flat_aggregated) = self.aggregate_builders(data.builders);
                    
                    // Add to total flat aggregation
                    for (extra_data, num_blocks) in &flat_aggregated {
                        *total_flat_aggregated.entry(extra_data.clone()).or_insert(0) += num_blocks;
                    }
                    
                    println!("  Found {} unique parent builders", hierarchical.len());
                    all_hierarchical_data.push(hierarchical);
                }
                Err(e) => {
                    println!("  Error: {}", e);
                }
            }
        }

        if all_hierarchical_data.is_empty() {
            anyhow::bail!("No data found for the specified date range");
        }

        // Merge hierarchical data
        let merged_hierarchical = self.merge_hierarchical_data(all_hierarchical_data);
        
        // Print results
        self.print_hierarchical_results(&merged_hierarchical);

        // Sort flat aggregated data by blocks (descending)
        let mut sorted_flat: Vec<_> = total_flat_aggregated.into_iter().collect();
        sorted_flat.sort_by(|a, b| b.1.cmp(&a.1));

        // Convert to ordered map for JSON
        let output_data: serde_json::Map<String, serde_json::Value> = sorted_flat
            .into_iter()
            .map(|(k, v)| (k, serde_json::Value::Number(v.into())))
            .collect();

        // Save to file
        let json = serde_json::to_string_pretty(&output_data)?;
        fs::write(output_path, json)
            .context("Failed to write output file")?;

        println!("\nFlat aggregated results saved to: {}", output_path.display());
        
        Ok(())
    }

    /// Print hierarchical results in a nice format
    fn print_hierarchical_results(&self, hierarchical_data: &[HierarchicalBuilder]) {
        println!("\n{}", "=".repeat(80));
        println!("AGGREGATED BUILDER RESULTS (Hierarchical)");
        println!("{}", "=".repeat(80));
        println!("{:<50} {:<10} {:<10}", "Builder Name", "Blocks", "Percentage");
        println!("{}", "-".repeat(80));

        // Calculate total blocks
        let total_blocks: u64 = hierarchical_data.iter().map(|b| b.blocks).sum();

        for builder in hierarchical_data {
            let name = if builder.name.is_empty() { "(empty)" } else { &builder.name };
            let percentage = if total_blocks > 0 {
                (builder.blocks as f64 / total_blocks as f64) * 100.0
            } else {
                0.0
            };

            println!("{:<50} {:<10} {:<9.2}%", name, builder.blocks, percentage);

            // Print children
            if !builder.children.is_empty() {
                for (i, child) in builder.children.iter().enumerate() {
                    let is_last = i == builder.children.len() - 1;
                    let prefix = if is_last { "└── " } else { "├── " };
                    let child_name = if child.name.is_empty() { "(empty)" } else { &child.name };
                    let child_percentage = if total_blocks > 0 {
                        (child.blocks as f64 / total_blocks as f64) * 100.0
                    } else {
                        0.0
                    };

                    let display_name = format!("{}{}", prefix, child_name);
                    println!("{:<50} {:<10} {:<9.2}%", display_name, child.blocks, child_percentage);
                }
            }
        }

        println!("{}", "-".repeat(80));
        println!("{:<50} {:<10} {:<10}", "TOTAL", total_blocks, "100.00%");
    }
}