#!/usr/bin/env python3
"""
Verify XSD schema file exists for OPNsense Config Faker.
"""

import sys
from pathlib import Path


def verify_xsd_file() -> bool:
    """Verify that opnsense-config.xsd exists in the project directory."""
    project_dir = Path(__file__).parent.parent
    xsd_file = project_dir / "opnsense-config.xsd"

    if not xsd_file.exists():
        print("ERROR: opnsense-config.xsd missing")
        print(f"Expected location: {xsd_file}")
        sys.exit(1)

    print(f"✅ Found XSD schema: {xsd_file}")
    return True


def main() -> None:
    """Main verification function."""
    print("🔍 Verifying XSD setup...")

    try:
        verify_xsd_file()
        print("✅ XSD setup verified!")
    except Exception as e:
        print(f"❌ XSD verification failed: {e}")
        sys.exit(1)


if __name__ == "__main__":
    main()
