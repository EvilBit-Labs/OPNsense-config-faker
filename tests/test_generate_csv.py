"""Unit tests for generate_csv module."""

from __future__ import annotations

import csv
from pathlib import Path
from typing import TYPE_CHECKING
from unittest.mock import patch

import pytest

from generate_csv import MAX_VLAN_COUNT, ConfigGenerationError, generate_csv

if TYPE_CHECKING:
    from _pytest.monkeypatch import MonkeyPatch

# Constants for magic numbers
EXPECTED_HEADER_PLUS_RECORDS = 6  # 5 records + 1 header
TEST_RECORD_COUNT = 5
LARGE_TEST_COUNT = 50


def test_generate_csv_creates_file(tmp_path: Path) -> None:
    """Test that CSV file is created with correct structure."""
    output_file = tmp_path / "test_output.csv"
    generate_csv(output_file, TEST_RECORD_COUNT)
    assert output_file.exists()  # noqa: S101

    # Verify file content structure
    with output_file.open(newline="") as f:
        reader = csv.reader(f)
        rows = list(reader)
    assert len(rows) == EXPECTED_HEADER_PLUS_RECORDS  # noqa: S101
    assert rows[0] == ["VLAN", "IP Range", "Beschreibung", "WAN"]  # noqa: S101


def test_generate_csv_minimum_records() -> None:
    """Test that ValueError is raised for invalid record counts."""
    with pytest.raises(ValueError, match="Number of records must be at least 1"):
        generate_csv("dummy.csv", 0)


def test_generate_csv_maximum_records(tmp_path: Path) -> None:
    """Test that maximum record count is allowed."""
    output_file = tmp_path / "max_output.csv"
    # Should allow MAX_VLAN_COUNT
    generate_csv(output_file, MAX_VLAN_COUNT)
    assert output_file.exists()  # noqa: S101


def test_generate_csv_exceeds_max_records_warning_only(tmp_path: Path) -> None:
    """Test that exceeding max records shows warning but still generates file."""
    output_file = tmp_path / "warn_output.csv"

    # Mock the Rich stderr console to capture output
    captured_output: list[str] = []

    def mock_print(text: str, **_kwargs: object) -> None:
        captured_output.append(text)

    with patch("generate_csv.stderr_console.print", side_effect=mock_print):
        generate_csv(output_file, MAX_VLAN_COUNT + 1)

    assert output_file.exists()  # noqa: S101

    # Check that warning was printed
    assert len(captured_output) == 1  # noqa: S101
    warning_text = captured_output[0]
    assert "exceeds practical VLAN limit" in warning_text  # noqa: S101
    assert "may have duplicate issues" in warning_text  # noqa: S101


def test_generate_csv_handles_os_error(monkeypatch: MonkeyPatch) -> None:
    """Test that OSError is properly handled and wrapped."""

    def raise_os_error(*_args: object, **_kwargs: object) -> None:
        raise OSError("Cannot write file")

    monkeypatch.setattr(Path, "open", raise_os_error)
    with pytest.raises(ConfigGenerationError):
        generate_csv("dummy.csv", 1)


def test_generate_csv_unique_vlans_and_networks(tmp_path: Path) -> None:
    """Test that generated VLANs and IP ranges are unique."""
    output_file = tmp_path / "unique.csv"
    generate_csv(output_file, LARGE_TEST_COUNT)
    with output_file.open(newline="") as f:
        reader = csv.DictReader(f)
        vlans: set[int] = set()
        ip_ranges: set[str] = set()
        for row in reader:
            vlan = int(row["VLAN"])
            ip_range = row["IP Range"]
            assert vlan not in vlans  # noqa: S101
            assert ip_range not in ip_ranges  # noqa: S101
            vlans.add(vlan)
            ip_ranges.add(ip_range)
