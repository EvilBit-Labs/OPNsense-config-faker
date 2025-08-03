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

import xml.etree.ElementTree as ET


def find_tag_position(input_xml, tag_to_find):
    with open(input_xml) as file:
        content = file.read()

    start_tag_start_pos = content.find(f"<{tag_to_find}")
    start_tag_end_pos = content.find(">", start_tag_start_pos) + 1  # Das Ende des Start-Tags
    end_tag_start_pos = content.find(f"</{tag_to_find}")  # Der Anfang des End-Tags

    return start_tag_end_pos, end_tag_start_pos


def insert_content_to_tag(input_xml, tag_to_insert, replacement_content):
    with open(input_xml) as file:
        content = file.read()

    start_pos, end_pos = find_tag_position(input_xml, tag_to_insert)
    content = content[:start_pos] + "\n" + replacement_content + content[end_pos:]

    with open(input_xml, "w") as file:
        file.write(content)


input_xml_file = "config-firewall1.dhw.xml"
tag_name = "vlans"
replacement_content_file = "part_VLAN.xml"

with open(replacement_content_file) as file:
    content = file.read()

insert_content_to_tag(input_xml_file, tag_name, content)
