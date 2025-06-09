#!/usr/bin/env python3
"""
Relay Data Aggregator

Fetches relay data from relayscan.io, aggregates builders by extra_data,
and outputs results both to console and JSON file.
"""

import requests
import json
from datetime import datetime, timedelta
from collections import defaultdict
import argparse
import sys

def fetch_relay_data(date_str):
    """
    Fetch relay data for a specific date from relayscan.io

    Args:
        date_str (str): Date in YYYY-MM-DD format

    Returns:
        dict: JSON response or None if failed
    """
    url = f"https://www.relayscan.io/stats/day/{date_str}/json"

    try:
        response = requests.get(url, timeout=10)
        response.raise_for_status()
        return response.json()
    except requests.exceptions.RequestException as e:
        print(f"Error fetching data for {date_str}: {e}")
        return None

def aggregate_builders(builders_data):
    """
    Aggregate builders by extra_data, preserving hierarchy

    Args:
        builders_data (list): List of builder objects

    Returns:
        tuple: (hierarchical_structure, flat_aggregated)
    """
    hierarchical = []
    flat_aggregated = defaultdict(int)

    for builder in builders_data:
        extra_data = builder['info']['extra_data'].strip()
        num_blocks = builder['info']['num_blocks']

        # Add to flat aggregation
        flat_aggregated[extra_data] += num_blocks

        # Create hierarchical structure
        builder_info = {
            'name': extra_data,
            'blocks': num_blocks,
            'children': []
        }

        # Add children if they exist
        if 'children' in builder and builder['children']:
            children = []
            for child in builder['children']:
                child_extra_data = child['extra_data'].strip()
                child_blocks = child['num_blocks']

                # Add to flat aggregation
                flat_aggregated[child_extra_data] += child_blocks

                children.append({
                    'name': child_extra_data,
                    'blocks': child_blocks
                })

            # Sort children by blocks (descending)
            children.sort(key=lambda x: x['blocks'], reverse=True)
            builder_info['children'] = children

        hierarchical.append(builder_info)

    # Sort parents by blocks (descending)
    hierarchical.sort(key=lambda x: x['blocks'], reverse=True)

    return hierarchical, dict(flat_aggregated)

def get_date_range(start_date=None, end_date=None, days=7):
    """
    Generate list of dates for the specified range

    Args:
        start_date (str): Start date in YYYY-MM-DD format
        end_date (str): End date in YYYY-MM-DD format
        days (int): Number of days to go back from today if no dates specified

    Returns:
        list: List of date strings
    """
    if start_date and end_date:
        start = datetime.strptime(start_date, '%Y-%m-%d')
        end = datetime.strptime(end_date, '%Y-%m-%d')

        if start > end:
            start, end = end, start

        dates = []
        current = start
        while current <= end:
            dates.append(current.strftime('%Y-%m-%d'))
            current += timedelta(days=1)
        return dates

    else:
        # Default: go back 'days' days from yesterday (excluding today)
        today = datetime.now()
        yesterday = today - timedelta(days=1)

        dates = []
        for i in range(days):
            date = yesterday - timedelta(days=i)
            dates.append(date.strftime('%Y-%m-%d'))

        return sorted(dates)

def print_hierarchical_results(hierarchical_data):
    """
    Print results in a nice hierarchical format with columns
    """
    print("\n" + "=" * 80)
    print("AGGREGATED BUILDER RESULTS (Hierarchical)")
    print("=" * 80)
    print(f"{'Builder Name':<50} {'Blocks':<10} {'Percentage':<10}")
    print("-" * 80)

    # Calculate total blocks for percentage calculation
    total_blocks = sum(builder['blocks'] for builder in hierarchical_data)

    for builder in hierarchical_data:
        # Print parent
        name = builder['name'] if builder['name'] else "(empty)"
        blocks = builder['blocks']
        percentage = (blocks / total_blocks * 100) if total_blocks > 0 else 0

        print(f"{name:<50} {blocks:<10,} {percentage:<9.2f}%")

        # Print children with ASCII hierarchy indicators
        if builder['children']:
            for i, child in enumerate(builder['children']):
                is_last = i == len(builder['children']) - 1
                prefix = "└── " if is_last else "├── "
                child_name = child['name'] if child['name'] else "(empty)"
                child_blocks = child['blocks']
                child_percentage = (child_blocks / total_blocks * 100) if total_blocks > 0 else 0

                display_name = f"{prefix}{child_name}"
                print(f"{display_name:<50} {child_blocks:<10,} {child_percentage:<9.2f}%")

    print("-" * 80)
    print(f"{'TOTAL':<50} {total_blocks:<10,} {'100.00%':<10}")

def merge_hierarchical_data(all_hierarchical_data):
    """
    Merge hierarchical data from multiple days
    """
    merged_builders = defaultdict(lambda: {'blocks': 0, 'children': defaultdict(int)})

    # Merge all data
    for daily_data in all_hierarchical_data:
        for builder in daily_data:
            name = builder['name']
            merged_builders[name]['blocks'] += builder['blocks']

            for child in builder['children']:
                child_name = child['name']
                merged_builders[name]['children'][child_name] += child['blocks']

    # Convert to final format
    result = []
    for name, data in merged_builders.items():
        builder_info = {
            'name': name,
            'blocks': data['blocks'],
            'children': []
        }

        # Add children if any
        if data['children']:
            children = []
            for child_name, child_blocks in data['children'].items():
                children.append({
                    'name': child_name,
                    'blocks': child_blocks
                })

            # Sort children by blocks (descending)
            children.sort(key=lambda x: x['blocks'], reverse=True)
            builder_info['children'] = children

        result.append(builder_info)

    # Sort parents by blocks (descending)
    result.sort(key=lambda x: x['blocks'], reverse=True)

    return result

def main():
    parser = argparse.ArgumentParser(description='Aggregate relay builder data from relayscan.io')
    parser.add_argument('--start', '-s', help='Start date (YYYY-MM-DD)')
    parser.add_argument('--end', '-e', help='End date (YYYY-MM-DD)')
    parser.add_argument('--days', '-d', type=int, default=7,
                        help='Number of days to fetch (default: 7, used when start/end not specified)')
    parser.add_argument('--output', '-o', default='builder_aggregated.json',
                        help='Output JSON file (default: builder_aggregated.json)')

    args = parser.parse_args()

    # Get date range
    dates = get_date_range(args.start, args.end, args.days)

    print(f"Fetching data for dates: {dates[0]} to {dates[-1]} ({len(dates)} days)")
    print("=" * 60)

    # Collect hierarchical data from all dates
    all_hierarchical_data = []
    total_flat_aggregated = defaultdict(int)

    for date_str in dates:
        print(f"Fetching data for {date_str}...")
        data = fetch_relay_data(date_str)

        if data and 'builders' in data:
            hierarchical, flat_aggregated = aggregate_builders(data['builders'])
            all_hierarchical_data.append(hierarchical)

            # Add to total flat aggregation for JSON output
            for extra_data, num_blocks in flat_aggregated.items():
                total_flat_aggregated[extra_data] += num_blocks

            print(f"  Found {len(hierarchical)} unique parent builders")
        else:
            print(f"  No data available for {date_str}")

    if not all_hierarchical_data:
        print("No data found for the specified date range.")
        sys.exit(1)

    # Merge hierarchical data from all days
    merged_hierarchical = merge_hierarchical_data(all_hierarchical_data)

    # Print hierarchical results
    print_hierarchical_results(merged_hierarchical)

    # Save flat aggregated data to JSON file
    sorted_flat = sorted(total_flat_aggregated.items(), key=lambda x: x[1], reverse=True)
    output_data = {extra_data: num_blocks for extra_data, num_blocks in sorted_flat}

    try:
        with open(args.output, 'w') as f:
            json.dump(output_data, f, indent=2, ensure_ascii=False)
        print(f"\nFlat aggregated results saved to: {args.output}")
    except IOError as e:
        print(f"Error saving to file: {e}")

    print(f"\nTotal unique builders: {len(total_flat_aggregated)}")
    print(f"Total blocks across all days: {sum(total_flat_aggregated.values()):,}")


if __name__ == "__main__":
    main()