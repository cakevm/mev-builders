#!/usr/bin/env python3
"""
Script to compare data/builders_stats.json with data/builders.json
and identify any mismatches between them.
"""

import json
import os
import sys
from typing import Dict, List, Set, Tuple


def load_json_file(filepath: str) -> dict:
    """Load and parse a JSON file."""
    with open(filepath, 'r') as f:
        return json.load(f)


def get_builders_extra_data(builders: List[dict]) -> Dict[str, dict]:
    """Extract builders indexed by their extra_data field."""
    builders_by_extra_data = {}
    for builder in builders:
        extra_data = builder.get('extra_data')
        if extra_data:
            builders_by_extra_data[extra_data] = builder
    return builders_by_extra_data


def compare_data(builders_file: str, stats_file: str) -> Tuple[List[str], List[str], List[str]]:
    """
    Compare builders.json with builders_stats.json.
    
    Returns:
        - List of extra_data keys in stats but not in builders
        - List of builders with extra_data not found in stats
        - List of builders without extra_data
    """
    # Load data
    builders = load_json_file(builders_file)
    stats = load_json_file(stats_file)
    
    # Get builders indexed by extra_data
    builders_by_extra_data = get_builders_extra_data(builders)
    
    # Get all extra_data values from builders
    builder_extra_data_set = set(builders_by_extra_data.keys())
    
    # Get all keys from stats
    stats_keys_set = set(stats.keys())
    
    # Find stats keys not in builders
    stats_not_in_builders = sorted(stats_keys_set - builder_extra_data_set)
    
    # Find builders with extra_data not in stats
    builders_not_in_stats = []
    builders_without_extra_data = []
    
    for builder in builders:
        extra_data = builder.get('extra_data')
        if extra_data is None:
            builders_without_extra_data.append(builder)
        elif extra_data not in stats:
            builders_not_in_stats.append(builder)
    
    return stats_not_in_builders, builders_not_in_stats, builders_without_extra_data


def main():
    """Main function."""
    # Get the repository root directory
    script_dir = os.path.dirname(os.path.abspath(__file__))
    repo_root = os.path.dirname(os.path.dirname(script_dir))
    
    # Paths to the JSON files
    builders_file = os.path.join(repo_root, 'crates', 'mev-builders', 'data', 'builders.json')
    stats_file = os.path.join(repo_root, 'crates', 'mev-builders', 'data', 'builders_stats.json')
    
    # Check if files exist
    if not os.path.exists(builders_file):
        print(f"Error: {builders_file} not found")
        sys.exit(1)
    
    if not os.path.exists(stats_file):
        print(f"Error: {stats_file} not found")
        sys.exit(1)
    
    # Compare the data
    stats_not_in_builders, builders_not_in_stats, builders_without_extra_data = compare_data(
        builders_file, stats_file
    )
    
    # Print results
    has_issues = False
    
    if stats_not_in_builders:
        has_issues = True
        print("❌ Stats entries not found in builders.json:")
        print("=" * 50)
        for key in stats_not_in_builders:
            stats = load_json_file(stats_file)
            blocks = stats[key]
            print(f"  - '{key}' ({blocks} blocks)")
        print()
    
    if builders_not_in_stats:
        has_issues = True
        print("❌ Builders with extra_data not found in builders_stats.json:")
        print("=" * 50)
        for builder in builders_not_in_stats:
            print(f"  - {builder['name']} (identifier: {builder['identifier']}, extra_data: '{builder['extra_data']}')")
        print()
    
    if builders_without_extra_data:
        print("⚠️  Builders without extra_data (cannot be matched with stats):")
        print("=" * 50)
        for builder in builders_without_extra_data:
            print(f"  - {builder['name']} (identifier: {builder['identifier']})")
        print()
    
    if not has_issues and not builders_without_extra_data:
        print("✅ All builders and stats are properly matched!")
    
    # Return exit code based on whether there are issues
    sys.exit(1 if has_issues else 0)


if __name__ == "__main__":
    main()