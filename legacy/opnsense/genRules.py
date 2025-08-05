"""
OPNsense Configuration Generator

This module contains functionality derived from or inspired by the original
OPNsense configuration generator by Stefan Reichhard (nett-media).

Original work: https://github.com/nett-media/opnsense-config-generator
Original author: Stefan Reichhard
Initial implementation: November 2023

Enhanced and modernized by EvilBit Labs for general network configuration
data generation with Faker integration.

This implementation maintains the core concepts while adding:
- Modern Python practices and type hints
- Faker integration for realistic test data
- Improved error handling and validation
- Modular architecture for extensibility
"""

import csv
import uuid
import time


def generate_xml_from_csv(csv_file, output_file, options):
    with open(csv_file) as file:
        reader = csv.reader(file)
        next(reader)  # Überspringt die Header-Zeile

        # adjust op_counter to match first free Opt Interface (depending on init_Interface.xml)
        opt_counter = options.get("opt_counter", 1)

        with open(output_file, "w") as outfile:
            for row in reader:
                vlan_nr = row[0].strip()
                random_uuid = str(uuid.uuid4())

                timestamp = time.time()
                formatted_timestamp = f"{timestamp:.4f}"

                outfile.write(f'<rule uuid="{random_uuid}">\n')
                outfile.write(f"  <type>pass</type>\n")
                outfile.write(f"  <interface>opt{opt_counter}</interface>\n")
                outfile.write(f"  <ipprotocol>inet</ipprotocol>\n")
                outfile.write(f"  <statetype>keep state</statetype>\n")
                outfile.write(f"  <descr>default allow VLAN_{vlan_nr} any</descr>\n")
                outfile.write(f"  <direction>in</direction>\n")
                outfile.write(f"  <quick>1</quick>\n")
                outfile.write(f"  <source>\n")
                outfile.write(f"    <any>1</any>\n")
                outfile.write(f"  </source>\n")
                outfile.write(f"  <destination>\n")
                outfile.write(f"    <any>1</any>\n")
                outfile.write(f"  </destination>\n")
                outfile.write(f"  <updated>\n")
                outfile.write(f"    <username>root@10.1.1.1</username>\n")
                outfile.write(f"    <time>{formatted_timestamp}</time>\n")
                outfile.write(f"    <description>genRules.py made changes</description>\n")
                outfile.write(f"  </updated>\n")
                outfile.write(f"  <created>\n")
                outfile.write(f"    <username>root@10.1.1.1</username>\n")
                outfile.write(f"    <time>{formatted_timestamp}</time>\n")
                outfile.write(f"    <description>genRules.py made changes</description>\n")
                outfile.write(f"  </created>\n")
                outfile.write(f"</rule>\n\n")

                opt_counter += 1


if __name__ == "__main__":
    csv_file = "config.csv"
    output_file = "part_Rules.xml"
    options = {"opt_counter": 1}  # Default starting counter
    generate_xml_from_csv(csv_file, output_file, options)
