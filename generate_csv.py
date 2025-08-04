#!/usr/bin/env python3
"""OPNsense Config Faker - CSV Generation Entry Point.

This is a compatibility wrapper that provides the expected generate_csv.py entry point
while delegating to the main module's CSV generation functionality.

The main application logic is in main.py with a modern Typer-based CLI interface.
This file exists to maintain compatibility with existing workflows and justfile recipes.
"""

import sys
from pathlib import Path

# Add the project root to the path so we can import main
project_root = Path(__file__).parent
sys.path.insert(0, str(project_root))

try:
    from main import app
except ImportError as e:
    print(f"Error importing main module: {e}", file=sys.stderr)
    print("Make sure you're running this from the project root directory.", file=sys.stderr)
    sys.exit(1)

if __name__ == "__main__":
    # Route to the CSV command in the main CLI app
    # Insert 'csv' as the first argument to route to the CSV subcommand
    sys.argv.insert(1, "csv")
    app()
