#!/usr/bin/env python3
"""
Script to reorder BUILDERS array based on block counts from builder_aggregated.json
Matches builders by their extra_data field and sorts by frequency (highest first)
"""

import json
import re
from typing import Dict, List, Tuple, Optional

def load_aggregated_data(filename: str) -> Dict[str, int]:
    """Load the aggregated builder data from JSON file"""
    try:
        with open(filename, 'r') as f:
            return json.load(f)
    except FileNotFoundError:
        print(f"Error: {filename} not found")
        return {}
    except json.JSONDecodeError:
        print(f"Error: Invalid JSON in {filename}")
        return {}

def parse_rust_builders(filename: str) -> List[Dict]:
    """Parse the Rust source file to extract BUILDERS array"""
    try:
        with open(filename, 'r') as f:
            builders_content = f.read()
    except FileNotFoundError:
        print(f"Error: {filename} not found")
        return []

    # Parse individual builders
    builders = []
    builder_pattern = r'Builder\s*\{(.*?)\},'

    for match in re.finditer(builder_pattern, builders_content, re.DOTALL):
        builder_content = match.group(1)
        builder = {}

        # Extract fields
        field_patterns = {
            'name': r'name:\s*"([^"]*)"',
            'identifier': r'identifier:\s*"([^"]*)"',
            'website': r'website:\s*"([^"]*)"',
            'searcher_rpc': r'searcher_rpc:\s*"([^"]*)"',
            'extra_data': r'extra_data:\s*Some\("([^"]*)"\)',
            'signing': r'signing:\s*Signing::(\w+)',
            'account_required': r'account_required:\s*(true|false)',
        }

        for field, pattern in field_patterns.items():
            match = re.search(pattern, builder_content)
            if match:
                if field == 'account_required':
                    builder[field] = match.group(1) == 'true'
                else:
                    builder[field] = match.group(1)
            elif field == 'extra_data':
                # Check for None
                if 'extra_data: None' in builder_content:
                    builder[field] = None
                else:
                    builder[field] = None

        # Handle mev_share_rpc (can be Some or None)
        mev_share_match = re.search(r'mev_share_rpc:\s*Some\("([^"]*)"\)', builder_content)
        if mev_share_match:
            builder['mev_share_rpc'] = mev_share_match.group(1)
        else:
            builder['mev_share_rpc'] = None

        builders.append(builder)

    return builders

def match_builder_to_count(builder: Dict, aggregated_data: Dict[str, int]) -> int:
    """Match a builder to its count in aggregated data based on extra_data"""
    extra_data = builder.get('extra_data')

    if extra_data is None:
        return 0

    # Direct match
    if extra_data in aggregated_data:
        return aggregated_data[extra_data]

    # Try some common variations
    variations = [
        extra_data.lower(),
        extra_data.upper(),
        extra_data.replace('https://', '').replace('http://', ''),
        extra_data.strip(),
    ]

    for variation in variations:
        if variation in aggregated_data:
            return aggregated_data[variation]

    return 0

def format_rust_builder(builder: Dict, indent: str = "    ") -> str:
    """Format a builder dict back to Rust struct format"""
    lines = [f"{indent}Builder {{"]

    # Required fields
    lines.append(f'{indent}    name: "{builder["name"]}",')
    lines.append(f'{indent}    identifier: "{builder["identifier"]}",')
    lines.append(f'{indent}    website: "{builder["website"]}",')
    lines.append(f'{indent}    searcher_rpc: "{builder["searcher_rpc"]}",')

    # Optional mev_share_rpc
    if builder.get('mev_share_rpc'):
        lines.append(f'{indent}    mev_share_rpc: Some("{builder["mev_share_rpc"]}"),')
    else:
        lines.append(f'{indent}    mev_share_rpc: None,')

    # Optional extra_data
    if builder.get('extra_data'):
        lines.append(f'{indent}    extra_data: Some("{builder["extra_data"]}"),')
    else:
        lines.append(f'{indent}    extra_data: None,')

    # Signing enum
    signing = builder.get('signing', 'NotSupported')
    lines.append(f'{indent}    signing: Signing::{signing},')

    # Account required
    account_req = str(builder.get('account_required', False)).lower()
    lines.append(f'{indent}    account_required: {account_req},')

    lines.append(f"{indent}}},")

    return "\n".join(lines)

def reorder_builders(builders: List[Dict], aggregated_data: Dict[str, int]) -> List[Tuple[Dict, int]]:
    """Reorder builders based on their counts"""
    builders_with_counts = []

    for builder in builders:
        count = match_builder_to_count(builder, aggregated_data)
        builders_with_counts.append((builder, count))

    # Sort by count (descending), then by name for ties
    builders_with_counts.sort(key=lambda x: (-x[1], x[0]['name']))

    return builders_with_counts

def generate_reordered_rust_code(builders_with_counts: List[Tuple[Dict, int]]) -> str:
    """Generate the complete Rust file with imports, BUILDERS and OTHER_BUILDERS arrays"""
    # Separate builders with and without matches
    matched_builders = [(builder, count) for builder, count in builders_with_counts if count > 0]
    unmatched_builders = [(builder, count) for builder, count in builders_with_counts if count == 0]

    lines = []

    # Add imports
    lines.append("use crate::{Builder, Signing};")
    lines.append("")

    # Generate BUILDERS array (matched builders only)
    lines.append("/// List of known builders with their details, ordered by block production.")
    lines.append("pub static BUILDERS: &[Builder] = &[")

    for builder, count in matched_builders:
        lines.append(f"    // Blocks: {count:,}")
        lines.append(format_rust_builder(builder))

    lines.append("];")
    lines.append("")

    # Generate OTHER_BUILDERS array (unmatched builders)
    lines.append("/// Other builders without recent block production data.")
    lines.append("pub static OTHER_BUILDERS: &[Builder] = &[")

    for builder, count in unmatched_builders:
        lines.append(f"    // No recent block data")
        lines.append(format_rust_builder(builder))

    lines.append("];")

    return "\n".join(lines)

def main():
    # Load aggregated data
    print("Loading builder aggregated data...")
    aggregated_data = load_aggregated_data('builder_aggregated.json')

    if not aggregated_data:
        return

    print(f"Loaded {len(aggregated_data)} entries from aggregated data")

    # Parse existing builders
    builders_file = '../../src/builders.rs'
    print(f"Parsing existing BUILDERS from {builders_file}...")
    builders = parse_rust_builders(builders_file)

    if not builders:
        return

    print(f"Parsed {len(builders)} builders from source")

    # Reorder builders
    print("Matching builders to counts and reordering...")
    builders_with_counts = reorder_builders(builders, aggregated_data)

    # Separate matched and unmatched
    matched_builders = [(builder, count) for builder, count in builders_with_counts if count > 0]
    unmatched_builders = [(builder, count) for builder, count in builders_with_counts if count == 0]

    # Show matching results
    print(f"\nMatching results:")
    print(f"  ✓ Matched builders: {len(matched_builders)}")
    print(f"  ✗ Unmatched builders: {len(unmatched_builders)}")
    print()

    print("Matched builders:")
    for builder, count in matched_builders:
        extra_data = builder.get('extra_data', 'None')
        print(f"  ✓ {builder['name']} ({extra_data}): {count:,} blocks")

    if unmatched_builders:
        print(f"\nUnmatched builders (will be placed after matched builders):")
        for builder, count in unmatched_builders:
            extra_data = builder.get('extra_data', 'None')
            print(f"  ✗ {builder['name']} ({extra_data}): No match")

    # Generate reordered code
    print(f"\nGenerating reordered Rust code...")
    reordered_code = generate_reordered_rust_code(builders_with_counts)

    # Write back to the same file
    print(f"Writing reordered code back to {builders_file}...")
    with open(builders_file, 'w') as f:
        f.write(reordered_code)

    print(f"✓ Successfully updated {builders_file}")
    print(f"  - BUILDERS array: {len(matched_builders)} builders (with block data)")
    print(f"  - OTHER_BUILDERS array: {len(unmatched_builders)} builders (without block data)")
    print(f"  - Total builders: {len(builders_with_counts)}")

    # Show top builders
    if matched_builders:
        print(f"\nTop builders by block count:")
        for i, (builder, count) in enumerate(matched_builders[:10]):
            print(f"  {i+1:2d}. {builder['name']:20s} - {count:,} blocks")

if __name__ == "__main__":
    main()