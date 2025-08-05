"""Unit tests for main module CSV functionality."""

from __future__ import annotations

import csv
from pathlib import Path
from typing import TYPE_CHECKING
from unittest.mock import patch

import pytest

from main import MAX_VLAN_COUNT, ConfigGenerationError, generate_vlan_configurations, save_to_csv

if TYPE_CHECKING:
    from _pytest.monkeypatch import MonkeyPatch

# Constants for magic numbers
EXPECTED_HEADER_PLUS_RECORDS = 6  # 5 records + 1 header
TEST_RECORD_COUNT = 5
LARGE_TEST_COUNT = 50


def test_generate_csv_creates_file(tmp_path: Path) -> None:
    """Test that CSV file is created with correct structure."""
    output_file = tmp_path / "test_output.csv"
    configs = generate_vlan_configurations(TEST_RECORD_COUNT)
    save_to_csv(configs, output_file)
    assert output_file.exists()

    # Verify file content structure
    with output_file.open(newline="") as f:
        reader = csv.reader(f)
        rows = list(reader)
    assert len(rows) == EXPECTED_HEADER_PLUS_RECORDS
    assert rows[0] == ["VLAN", "IP Range", "Beschreibung", "WAN"]


def test_generate_csv_minimum_records() -> None:
    """Test that ValueError is raised for invalid record counts."""
    with pytest.raises(ValueError, match="Number of configurations must be at least 1"):
        generate_vlan_configurations(0)


def test_generate_csv_maximum_records(tmp_path: Path) -> None:
    """Test that maximum record count is allowed."""
    output_file = tmp_path / "max_output.csv"
    # Should allow MAX_VLAN_COUNT
    configs = generate_vlan_configurations(MAX_VLAN_COUNT)
    save_to_csv(configs, output_file)
    assert output_file.exists()


def test_generate_csv_exceeds_max_records_warning_only(tmp_path: Path) -> None:
    """Test that exceeding max records shows warning but still generates file."""
    output_file = tmp_path / "warn_output.csv"

    # Mock the Rich stderr console to capture output
    captured_output: list[str] = []

    def mock_print(text: str, **_kwargs: object) -> None:
        captured_output.append(text)

    with patch("main.stderr_console.print", side_effect=mock_print):
        configs = generate_vlan_configurations(MAX_VLAN_COUNT + 1)
        save_to_csv(configs, output_file)

    assert output_file.exists()

    # Check that warning was printed
    assert len(captured_output) == 1
    warning_text = captured_output[0]
    assert "exceeds practical VLAN limit" in warning_text
    assert "may have duplicate issues" in warning_text


def test_generate_csv_handles_os_error(monkeypatch: MonkeyPatch) -> None:
    """Test that OSError is properly handled and wrapped."""

    def raise_os_error(*_args: object, **_kwargs: object) -> None:
        raise OSError("Cannot write file")

    configs = generate_vlan_configurations(1)
    monkeypatch.setattr(Path, "open", raise_os_error)
    with pytest.raises(ConfigGenerationError):
        save_to_csv(configs, Path("dummy.csv"))


def test_generate_csv_unique_vlans_and_networks(tmp_path: Path) -> None:
    """Test that generated VLANs and IP ranges are unique."""
    output_file = tmp_path / "unique.csv"
    configs = generate_vlan_configurations(LARGE_TEST_COUNT)
    save_to_csv(configs, output_file)
    with output_file.open(newline="") as f:
        reader = csv.DictReader(f)
        vlans: set[int] = set()
        ip_ranges: set[str] = set()
        for row in reader:
            vlan = int(row["VLAN"])
            ip_range = row["IP Range"]
            assert vlan not in vlans
            assert ip_range not in ip_ranges
            vlans.add(vlan)
            ip_ranges.add(ip_range)
