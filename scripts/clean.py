#!/usr/bin/env python3
"""
Clean up generated files and caches for OPNsense Config Faker.
"""

import shutil
from pathlib import Path


def _remove_directory_safe(path: Path) -> None:
    """Safely remove a directory with error handling."""
    try:
        shutil.rmtree(path)
        print(f"Removed: {path}")
    except Exception as e:
        print(f"Warning: Could not remove {path}: {e}")


def _remove_file_safe(path: Path) -> None:
    """Safely remove a file with error handling."""
    try:
        path.unlink()
        print(f"Removed: {path}")
    except Exception as e:
        print(f"Warning: Could not remove {path}: {e}")


def clean_pycache() -> None:
    """Remove all __pycache__ directories and .pyc/.pyo files."""
    # Remove __pycache__ directories
    pycache_dirs = list(Path().rglob("__pycache__"))
    for pycache_dir in pycache_dirs:
        _remove_directory_safe(pycache_dir)

    # Remove .pyc and .pyo files
    pyc_files = list(Path().rglob("*.pyc")) + list(Path().rglob("*.pyo"))
    for pyc_file in pyc_files:
        _remove_file_safe(pyc_file)


def clean_build_artifacts() -> None:
    """Remove build artifacts and cache directories."""
    dirs_to_remove = [".pytest_cache", "htmlcov", "build", "dist"]
    files_to_remove = list(Path().glob("*.egg-info"))

    # Remove directories
    for dir_name in dirs_to_remove:
        dir_path = Path(dir_name)
        if dir_path.exists():
            _remove_directory_safe(dir_path)

    # Remove egg-info files
    for egg_info in files_to_remove:
        if egg_info.exists():
            _remove_file_safe(egg_info)

    # Remove .coverage file
    coverage_file = Path(".coverage")
    if coverage_file.exists():
        _remove_file_safe(coverage_file)


def main() -> None:
    """Main cleaning function."""
    print("🧹 Cleaning .pyc files, __pycache__, and build artifacts...")

    # Change to the project directory
    project_dir = Path(__file__).parent.parent
    import os

    os.chdir(project_dir)

    clean_pycache()
    clean_build_artifacts()

    print("✅ Cleaning complete!")


if __name__ == "__main__":
    main()
