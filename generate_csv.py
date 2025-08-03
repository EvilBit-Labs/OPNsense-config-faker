"""OPNsense Config Faker - Network Configuration Data Generator.

Generates realistic network configuration test data for testing network automation tools,
configuration management systems, and network infrastructure projects.

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

from __future__ import annotations

import csv
import ipaddress
import sys
from pathlib import Path
from typing import Annotated

import typer
from faker import Faker
from rich.console import Console
from rich.progress import Progress, SpinnerColumn, TextColumn
from rich.prompt import Confirm

# Initialize global instances
fake = Faker()
console = Console()
stderr_console = Console(file=sys.stderr)

# Constants
MAX_VLAN_COUNT = 4084  # Maximum practical VLAN count (4094 - 10 reserved VLANs)
DEFAULT_OUTPUT_PATH = Path("output/test-config.csv")


class ConfigGenerationError(Exception):
    """Raised when CSV generation fails."""


def generate_csv(filename: str | Path, num_records: int) -> None:
    """Generate CSV file with network configuration data.

    Args:
        filename: Output CSV filename/path
        num_records: Number of VLAN configuration records to generate

    Raises:
        OSError: If file cannot be written
        ValueError: If num_records is invalid
        ConfigGenerationError: If generation process fails
    """
    if num_records < 1:
        raise ValueError("Number of records must be at least 1")

    if num_records > MAX_VLAN_COUNT:
        stderr_console.print(
            f"[yellow]Warning:[/yellow] Requested count ({num_records}) exceeds practical VLAN limit "
            f"({MAX_VLAN_COUNT}), may have duplicate issues"
        )
    try:
        with Path(filename).open(mode="w", newline="") as file:
            writer = csv.writer(file)
            writer.writerow(["VLAN", "IP Range", "Beschreibung", "WAN"])

            # Keep track of used VLANs and networks to avoid duplicates
            used_vlans: set[int] = set()
            used_networks: set[str] = set()

            # Use Rich progress bar for better UX
            with Progress(
                SpinnerColumn(),
                TextColumn("[progress.description]{task.description}"),
                console=console,
                transient=True,
            ) as progress:
                task = progress.add_task("Generating VLAN configurations...", total=num_records)

                for _ in range(num_records):
                    # Generate unique VLAN ID
                    while True:
                        vlan = fake.random_int(min=10, max=4094)  # Valid VLAN range
                        if vlan not in used_vlans:
                            used_vlans.add(vlan)
                            break

                    # Generate a unique private IPv4 network
                    while True:
                        # Generate a private IPv4 address using faker
                        private_ip = fake.ipv4_private()

                        # Convert to network object and get the first 3 octets
                        ip_obj = ipaddress.IPv4Address(private_ip)
                        octets = str(ip_obj).split(".")
                        network_base = f"{octets[0]}.{octets[1]}.{octets[2]}.x"

                        # Ensure we don't generate duplicate networks
                        if network_base not in used_networks:
                            used_networks.add(network_base)
                            ip_range = network_base
                            break

                    # Generate a more realistic description
                    department = fake.random_element(
                        elements=(
                            "Sales",
                            "IT",
                            "HR",
                            "Finance",
                            "Marketing",
                            "Operations",
                            "Engineering",
                            "Support",
                            "Admin",
                            "Guest",
                            "Lab",
                            "Test",
                        )
                    )
                    description = f"{department}{vlan}"

                    # WAN assignment (1-3 as per example)
                    wan = fake.random_int(min=1, max=3)

                    writer.writerow([vlan, ip_range, description, wan])
                    progress.advance(task)

    except OSError as e:
        raise ConfigGenerationError(f"Failed to write CSV file '{filename}': {e}") from e
    except Exception as e:
        raise ConfigGenerationError(f"Unexpected error during CSV generation: {e}") from e


# Initialize Typer app
app = typer.Typer(
    name="opnsense-config-faker",
    help="Generate realistic network configuration test data for OPNsense testing",
    epilog="""Examples:

  python generate_csv.py                    Generate 10 VLANs (default)
  python generate_csv.py --count 25         Generate 25 VLANs
  python generate_csv.py -c 50 -o my.csv   Generate 50 VLANs to my.csv
  python generate_csv.py --help             Show this help message
    """,
    rich_markup_mode="rich",
)


@app.command()
def main(
    count: Annotated[
        int,
        typer.Option(
            "-c",
            "--count",
            help="Number of VLAN configurations to generate",
            min=1,
            max=MAX_VLAN_COUNT,
            show_default=True,
        ),
    ] = 10,
    output: Annotated[
        Path,
        typer.Option(
            "-o",
            "--output",
            help="Output CSV filename",
            exists=False,
            file_okay=True,
            dir_okay=False,
            writable=True,
            resolve_path=True,
            show_default=True,
        ),
    ] = DEFAULT_OUTPUT_PATH,
    force: Annotated[
        bool,
        typer.Option(
            "-f",
            "--force",
            help="Overwrite existing output file without confirmation",
        ),
    ] = False,
) -> None:
    """Generate CSV file with realistic network configuration test data.

    Creates VLAN configurations with unique IDs, private IP ranges,
    department-based descriptions, and WAN assignments suitable for
    testing network automation tools and configuration management systems.
    """
    try:
        # Validate input parameters
        if count < 1:
            stderr_console.print("[red]Error:[/red] Count must be at least 1")
            raise typer.Exit(1)

        if count > MAX_VLAN_COUNT:
            stderr_console.print(
                f"[yellow]Warning:[/yellow] Requested count ({count}) exceeds practical VLAN limit "
                f"({MAX_VLAN_COUNT}), may have duplicate issues"
            )

        # Handle existing file
        if output.exists() and not force:
            console.print(f"[yellow]File '{output}' already exists.[/yellow]")
            if not Confirm.ask("Overwrite", default=False):
                console.print("[blue]Operation cancelled.[/blue]")
                raise typer.Exit(0)

        # Ensure output directory exists
        output.parent.mkdir(parents=True, exist_ok=True)

        # Display generation info
        console.print(f"[green]Generating {count} VLAN configurations...[/green]")
        console.print(f"[blue]Output file:[/blue] {output}")

        # Generate the CSV
        generate_csv(output, count)

        # Success message
        console.print(f"[green]✓ Successfully generated {count} VLAN configurations in {output}[/green]")

    except ConfigGenerationError as e:
        stderr_console.print(f"[red]Generation Error:[/red] {e}")
        raise typer.Exit(1) from e
    except ValueError as e:
        stderr_console.print(f"[red]Validation Error:[/red] {e}")
        raise typer.Exit(1) from e
    except KeyboardInterrupt:
        console.print("\n[yellow]Operation cancelled by user.[/yellow]")
        raise typer.Exit(130) from None
    except Exception as e:
        stderr_console.print(f"[red]Unexpected Error:[/red] {e}")
        raise typer.Exit(1) from e


if __name__ == "__main__":
    app()
