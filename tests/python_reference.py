#!/usr/bin/env python3
"""
Python Reference Implementation for Migration Validation

This module provides a Python reference implementation that generates
CSV data compatible with the legacy OPNsense Config Generator for
comparison testing with the Rust implementation.

This is specifically designed for migration validation and parity testing.
"""

import csv
import random
import sys
from typing import List, Tuple, Optional
from pathlib import Path

class PythonVlanGenerator:
    """Python reference implementation for VLAN configuration generation."""

    def __init__(self, seed: Optional[int] = None):
        """Initialize the generator with an optional seed for deterministic output."""
        self.seed = seed
        if seed is not None:
            random.seed(seed)

        # Track used values to ensure uniqueness
        self.used_vlan_ids = set()
        self.used_networks = set()

        # Define RFC1918 network ranges for class B networks
        self.class_b_ranges = [
            (172, 16, 31),   # 172.16.0.0/12
            (10, 0, 255),    # 10.0.0.0/8 (class B portion)
        ]

        # Department names for realistic descriptions
        self.departments = [
            "Sales", "Marketing", "Finance", "HR", "IT", "Engineering",
            "Development", "Security", "Research", "Operations", "Support"
        ]

    def generate_vlan_id(self) -> int:
        """Generate a unique VLAN ID in the valid range [10, 4094]."""
        max_attempts = 1000
        for _ in range(max_attempts):
            vlan_id = random.randint(10, 4094)
            if vlan_id not in self.used_vlan_ids:
                self.used_vlan_ids.add(vlan_id)
                return vlan_id
        raise RuntimeError("Unable to generate unique VLAN ID")

    def generate_ip_network(self) -> str:
        """Generate a unique RFC1918 network in x.y.z.x format."""
        max_attempts = 1000
        for _ in range(max_attempts):
            # Select random class B range
            if random.random() < 0.7:  # 70% class B 172.16.x.x
                base_octet = 172
                second_octet = random.randint(16, 31)
            else:  # 30% class A 10.x.x.x
                base_octet = 10
                second_octet = random.randint(0, 255)

            third_octet = random.randint(0, 255)
            network = f"{base_octet}.{second_octet}.{third_octet}"

            if network not in self.used_networks:
                self.used_networks.add(network)
                return f"{network}.x"
        raise RuntimeError("Unable to generate unique IP network")

    def generate_description(self, vlan_id: int) -> str:
        """Generate a realistic description for the VLAN."""
        department = random.choice(self.departments)
        return f"{department} VLAN {vlan_id}"

    def generate_wan_assignment(self) -> int:
        """Generate a WAN assignment (1, 2, or 3)."""
        return random.randint(1, 3)

    def generate_csv_data(self, count: int) -> List[Tuple[int, str, str, int]]:
        """Generate CSV data compatible with the legacy format."""
        if count > 4085:  # 4094 - 10 + 1 = max possible unique VLAN IDs
            raise ValueError(f"Cannot generate {count} unique VLAN configurations (max: 4085)")

        data = []
        for _ in range(count):
            vlan_id = self.generate_vlan_id()
            ip_range = self.generate_ip_network()
            description = self.generate_description(vlan_id)
            wan = self.generate_wan_assignment()
            data.append((vlan_id, ip_range, description, wan))

        return data

    def write_csv_file(self, filename: str, count: int) -> None:
        """Write CSV file with the specified number of configurations."""
        data = self.generate_csv_data(count)

        with open(filename, 'w', newline='', encoding='utf-8') as csvfile:
            writer = csv.writer(csvfile)
            # Write header exactly as expected
            writer.writerow(['VLAN', 'IP Range', 'Beschreibung', 'WAN'])

            # Write data rows
            for vlan_id, ip_range, description, wan in data:
                writer.writerow([vlan_id, ip_range, description, wan])

def main():
    """Command line interface for the Python reference implementation."""
    if len(sys.argv) != 4:
        print("Usage: python python_reference.py <count> <output_file> <seed>")
        sys.exit(1)

    try:
        count = int(sys.argv[1])
        output_file = sys.argv[2]
        seed = int(sys.argv[3]) if sys.argv[3] != "None" else None

        generator = PythonVlanGenerator(seed=seed)
        generator.write_csv_file(output_file, count)

        print(f"Generated {count} VLAN configurations in '{output_file}'")

    except ValueError as e:
        print(f"Error: {e}")
        sys.exit(1)
    except Exception as e:
        print(f"Unexpected error: {e}")
        sys.exit(1)

if __name__ == "__main__":
    main()
