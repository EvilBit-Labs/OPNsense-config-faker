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

import sys
from .string_utils import escape_html_entities


def escape_string(s):
    return escape_html_entities(s)


if __name__ == "__main__":
    # Prüfen, ob ein Argument angegeben wurde
    if len(sys.argv) < 2:
        print("Bitte geben Sie einen String als Argument an.")
        sys.exit(1)

    input_string = sys.argv[1]
    escaped = escape_string(input_string)
    print(escaped)
