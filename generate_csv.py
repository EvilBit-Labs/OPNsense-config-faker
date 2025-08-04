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
- Integrated XML generation for OPNsense configurations
"""

from __future__ import annotations

import csv
import ipaddress
import random
import shutil
import string
import sys
import time
import uuid
from pathlib import Path
from typing import Annotated, Any
from xml.sax.saxutils import escape

import typer
from faker import Faker

try:
    from lxml import etree  # type: ignore[attr-defined]
except ImportError:
    etree = None  # type: ignore[assignment]

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


class XMLGenerationError(Exception):
    """Raised when XML generation fails."""


def escape_xml_string(s: str) -> str:
    """Escape special characters for XML output.

    This function is derived from the original sanitizeDescription.py
    from Stefan Reichhard's OPNsense configuration generator.

    Args:
        s: String to escape

    Returns:
        Escaped string safe for XML
    """
    return escape(
        s, {"ä": "ae", "ö": "oe", "ü": "ue", "ß": "ss", "Ä": "AE", "Ö": "OE", "Ü": "UE", " ": "", "-": "_", "/": "_"}
    )


def generate_vlan_xml(csv_file: Path, output_file: Path, options: dict[str, Any]) -> None:  # noqa: ARG001
    """Generate VLAN XML configuration from CSV data.

    This function is derived from the original genVLAN.py
    from Stefan Reichhard's OPNsense configuration generator.

    Args:
        csv_file: Input CSV file path
        output_file: Output XML file path
        options: Configuration options
    """
    with csv_file.open() as file:
        reader = csv.reader(file)
        next(reader)  # Skip header row

        with output_file.open("w") as outfile:
            for row in reader:
                vlan_nr = row[0].strip()
                vlan_uuid = str(uuid.uuid4())
                descr = row[2].strip()

                outfile.write(f'    <vlan uuid="{vlan_uuid}">\n')
                outfile.write("      <if>lagg0</if>\n")
                outfile.write(f"      <tag>{vlan_nr}</tag>\n")
                outfile.write("      <pcp>0</pcp>\n")
                outfile.write("      <proto/>\n")
                outfile.write(f"      <descr>{descr}</descr>\n")
                outfile.write(f"      <vlanif>vlan0{vlan_nr}</vlanif>\n")
                outfile.write("    </vlan>\n")


def generate_interface_xml(csv_file: Path, output_file: Path, options: dict[str, Any]) -> None:
    """Generate Interface XML configuration from CSV data.

    This function is derived from the original genInterface.py
    from Stefan Reichhard's OPNsense configuration generator.

    Args:
        csv_file: Input CSV file path
        output_file: Output XML file path
        options: Configuration options
    """
    with csv_file.open() as file:
        reader = csv.reader(file)
        next(reader)  # Skip header row

        opt_counter: int = options.get("opt_counter", 1)
        firewall_number: int = options.get("firewallNr", 1)
        ip_suffix: int = 250 + firewall_number

        with output_file.open("w") as outfile:
            for row in reader:
                vlan_nr = row[0].strip()
                ip_address = row[1].strip().replace(".x", f".{ip_suffix}")
                description = escape_xml_string(row[2].strip())

                outfile.write(f"<opt{opt_counter}>\n")
                outfile.write(f"  <if>vlan0{vlan_nr}</if>\n")
                outfile.write(f"  <descr>V{vlan_nr}_{description}</descr>\n")
                outfile.write("  <enable>1</enable>\n")
                outfile.write("  <spoofmac/>\n")
                outfile.write(f"  <ipaddr>{ip_address}</ipaddr>\n")
                outfile.write("  <subnet>24</subnet>\n")
                outfile.write(f"</opt{opt_counter}>\n")

                opt_counter += 1


def generate_dhcp_xml(csv_file: Path, output_file: Path, options: dict[str, Any]) -> None:
    """Generate DHCP XML configuration from CSV data.

    This function is derived from the original genDHCP.py
    from Stefan Reichhard's OPNsense configuration generator.

    Args:
        csv_file: Input CSV file path
        output_file: Output XML file path
        options: Configuration options
    """
    with csv_file.open() as file:
        reader = csv.reader(file)
        next(reader)  # Skip header row

        opt_counter: int = options.get("opt_counter", 1)

        with output_file.open("w") as outfile:
            for row in reader:
                ip_range_base = row[1].strip().rsplit(".", 1)[0]

                dhcp_from = f"{ip_range_base}.1"
                dhcp_to = f"{ip_range_base}.100"
                gateway = f"{ip_range_base}.254"
                dns_server = f"{ip_range_base}.254"
                failover_peer = f"{ip_range_base}.252"

                outfile.write(f"<opt{opt_counter}>\n")
                outfile.write("  <enable>1</enable>\n")
                outfile.write(f"  <failover_peerip>{failover_peer}</failover_peerip>\n")
                outfile.write(f"  <gateway>{gateway}</gateway>\n")
                outfile.write("  <ddnsdomainalgorithm>hmac-md5</ddnsdomainalgorithm>\n")
                outfile.write("  <numberoptions>\n    <item/>\n  </numberoptions>\n")
                outfile.write(f"  <range>\n    <from>{dhcp_from}</from>\n    <to>{dhcp_to}</to>\n  </range>\n")
                outfile.write("  <winsserver/>\n")
                outfile.write(f"  <dnsserver>{dns_server}</dnsserver>\n")
                outfile.write("  <ntpserver/>\n")
                outfile.write(f"</opt{opt_counter}>\n")

                opt_counter += 1


def generate_rules_xml(csv_file: Path, output_file: Path, options: dict[str, Any]) -> None:
    """Generate firewall rules XML configuration from CSV data.

    This function is derived from the original genRules.py
    from Stefan Reichhard's OPNsense configuration generator.

    Args:
        csv_file: Input CSV file path
        output_file: Output XML file path
        options: Configuration options
    """
    with csv_file.open() as file:
        reader = csv.reader(file)
        next(reader)  # Skip header row

        opt_counter: int = options.get("opt_counter", 1)

        with output_file.open("w") as outfile:
            for row in reader:
                vlan_nr = row[0].strip()
                random_uuid = str(uuid.uuid4())

                timestamp = time.time()
                formatted_timestamp = f"{timestamp:.4f}"

                outfile.write(f'<rule uuid="{random_uuid}">\n')
                outfile.write("  <type>pass</type>\n")
                outfile.write(f"  <interface>opt{opt_counter}</interface>\n")
                outfile.write("  <ipprotocol>inet</ipprotocol>\n")
                outfile.write("  <statetype>keep state</statetype>\n")
                outfile.write(f"  <descr>default allow VLAN_{vlan_nr} any</descr>\n")
                outfile.write("  <direction>in</direction>\n")
                outfile.write("  <quick>1</quick>\n")
                outfile.write("  <source>\n")
                outfile.write("    <any>1</any>\n")
                outfile.write("  </source>\n")
                outfile.write("  <destination>\n")
                outfile.write("    <any>1</any>\n")
                outfile.write("  </destination>\n")
                outfile.write("  <updated>\n")
                outfile.write("    <username>root@10.1.1.1</username>\n")
                outfile.write(f"    <time>{formatted_timestamp}</time>\n")
                outfile.write("    <description>OPNsense Config Faker generated</description>\n")
                outfile.write("  </updated>\n")
                outfile.write("  <created>\n")
                outfile.write("    <username>root@10.1.1.1</username>\n")
                outfile.write(f"    <time>{formatted_timestamp}</time>\n")
                outfile.write("    <description>OPNsense Config Faker generated</description>\n")
                outfile.write("  </created>\n")
                outfile.write("</rule>\n\n")

                opt_counter += 1


def generate_nat_xml(csv_file: Path, output_file: Path, options: dict[str, Any]) -> None:
    """Generate NAT XML configuration from CSV data.

    This function is derived from the original genNAT.py
    from Stefan Reichhard's OPNsense configuration generator.

    Args:
        csv_file: Input CSV file path
        output_file: Output XML file path
        options: Configuration options
    """
    with csv_file.open() as file:
        reader = csv.reader(file)
        next(reader)  # Skip header row

        opt_counter: int = options.get("opt_counter", 1)

        with output_file.open("w") as outfile:
            for row in reader:
                description = row[2].strip()
                wan = row[3].strip()

                timestamp = time.time()
                formatted_timestamp = f"{timestamp:.4f}"

                wan_ip: str = ""
                if wan == "1":
                    wan_ip = options.get("wan1", "80.200.10.11")
                elif wan == "2":
                    wan_ip = options.get("wan2", "80.200.10.12")
                elif wan == "3":
                    wan_ip = options.get("wan3", "80.200.10.13")

                outfile.write("<rule>\n")
                outfile.write("  <source>\n")
                outfile.write(f"    <network>opt{opt_counter}</network>\n")
                outfile.write("  </source>\n")
                outfile.write("  <destination>\n")
                outfile.write("    <any>1</any>\n")
                outfile.write("  </destination>\n")
                outfile.write(f"  <descr>{description}</descr>\n")
                outfile.write("  <category/>\n")
                outfile.write("  <interface>wan</interface>\n")
                outfile.write("  <tag/>\n")
                outfile.write("  <tagged/>\n")
                outfile.write("  <poolopts/>\n")
                outfile.write("  <poolopts_sourcehashkey/>\n")
                outfile.write("  <ipprotocol>inet</ipprotocol>\n")
                outfile.write("  <created>\n")
                outfile.write("    <username>root@10.1.1.1</username>\n")
                outfile.write(f"    <time>{formatted_timestamp}</time>\n")
                outfile.write("    <description>OPNsense Config Faker generated</description>\n")
                outfile.write("  </created>\n")
                outfile.write(f"  <target>{wan_ip}</target>\n")
                outfile.write("  <sourceport/>\n")
                outfile.write("  <updated>\n")
                outfile.write("    <username>root@10.1.1.1</username>\n")
                outfile.write(f"    <time>{formatted_timestamp}</time>\n")
                outfile.write("    <description>OPNsense Config Faker generated</description>\n")
                outfile.write("  </updated>\n")
                outfile.write("</rule>\n")

                opt_counter += 1


def generate_random_password(length: int = 32) -> str:
    """Generate a random password for CARP.

    This function is derived from the original genCARP.py
    from Stefan Reichhard's OPNsense configuration generator.

    Args:
        length: Length of password to generate

    Returns:
        Random password string
    """
    characters = string.ascii_letters + string.digits
    return "".join(random.choice(characters) for _ in range(length))  # noqa: S311


def generate_carp_xml(csv_file: Path, output_file: Path, options: dict[str, Any]) -> None:
    """Generate CARP virtual IP XML configuration from CSV data.

    This function is derived from the original genCARP.py
    from Stefan Reichhard's OPNsense configuration generator.

    Args:
        csv_file: Input CSV file path
        output_file: Output XML file path
        options: Configuration options
    """
    with csv_file.open() as file:
        reader = csv.reader(file)
        next(reader)  # Skip header row

        opt_counter: int = options.get("opt_counter", 1)
        firewall_number: int = options.get("firewallNr", 1)
        advskew = "0" if firewall_number == 1 else "100"

        with output_file.open("w") as outfile:
            for row in reader:
                vlan_nr = row[0].strip()
                ip_range = row[1].strip().replace(".x", ".254")
                description = row[2].strip()
                vip_uuid = str(uuid.uuid4())
                random_password = generate_random_password()

                outfile.write(f'  <vip uuid="{vip_uuid}">\n')
                outfile.write(f"    <interface>opt{opt_counter}</interface>\n")
                outfile.write("    <mode>carp</mode>\n")
                outfile.write(f"    <subnet>{ip_range}</subnet>\n")
                outfile.write("    <subnet_bits>24</subnet_bits>\n")
                outfile.write("    <gateway/>\n")
                outfile.write("    <noexpand>0</noexpand>\n")
                outfile.write("    <nobind>0</nobind>\n")
                outfile.write(f"    <password>{random_password}</password>\n")
                outfile.write(f"    <vhid>{vlan_nr}</vhid>\n")
                outfile.write("    <advbase>1</advbase>\n")
                outfile.write(f"    <advskew>{advskew}</advskew>\n")
                outfile.write(f"    <descr>{description}</descr>\n")
                outfile.write("  </vip>\n")

                opt_counter += 1


def generate_radius_user_xml(csv_file: Path, output_file: Path, options: dict[str, Any]) -> None:  # noqa: ARG001
    """Generate RADIUS user XML configuration from CSV data.

    This function is derived from the original genRadiusUser.py
    from Stefan Reichhard's OPNsense configuration generator.

    Args:
        csv_file: Input CSV file path
        output_file: Output XML file path
        options: Configuration options
    """
    with csv_file.open() as file:
        reader = csv.reader(file)
        next(reader)  # Skip header row

        with output_file.open("w") as outfile:
            for row in reader:
                vlan_nr = row[0].strip()
                description = row[2].strip()
                user_uuid = str(uuid.uuid4())

                username = f"top{vlan_nr}"
                password = escape_xml_string(description)

                outfile.write(f'        <user uuid="{user_uuid}">\n')
                outfile.write("          <enabled>1</enabled>\n")
                outfile.write(f"          <username>{username}</username>\n")
                outfile.write(f"          <password>{password}</password>\n")
                outfile.write(f"          <description>{description}</description>\n")
                outfile.write("          <ip/>\n")
                outfile.write("          <subnet/>\n")
                outfile.write("          <route/>\n")
                outfile.write("          <ip6/>\n")
                outfile.write(f"          <vlan>{vlan_nr}</vlan>\n")
                outfile.write("          <logintime/>\n")
                outfile.write("          <simuse/>\n")
                outfile.write("          <exos_vlan_untagged/>\n")
                outfile.write("          <exos_vlan_tagged/>\n")
                outfile.write("          <exos_policy/>\n")
                outfile.write("          <wispr_bw_min_up/>\n")
                outfile.write("          <wispr_bw_max_up/>\n")
                outfile.write("          <wispr_bw_min_down/>\n")
                outfile.write("          <wispr_bw_max_down/>\n")
                outfile.write("          <chillispot_bw_max_up/>\n")
                outfile.write("          <chillispot_bw_max_down/>\n")
                outfile.write("          <mikrotik_vlan_id_number/>\n")
                outfile.write("          <mikrotik_vlan_id_type/>\n")
                outfile.write("          <sessionlimit_max_session_limit/>\n")
                outfile.write("          <servicetype/>\n")
                outfile.write("          <linkedAVPair/>\n")
                outfile.write("        </user>\n")


def modify_xml_config(input_xml: Path, tag_path: str, file_names: list[Path]) -> None:
    """Modify XML configuration by injecting generated parts.

    This function is derived from the original replaceTags.py
    from Stefan Reichhard's OPNsense configuration generator.

    Args:
        input_xml: Main XML config file to modify
        tag_path: XPath to the target element
        file_names: List of XML files to inject

    Raises:
        XMLGenerationError: If XML processing fails
    """
    try:
        parser: etree.XMLParser = etree.XMLParser(remove_blank_text=True)  # type: ignore[attr-defined]
        tree: etree._ElementTree = etree.parse(str(input_xml), parser)  # type: ignore[attr-defined]
        root: etree._Element = tree.getroot()  # type: ignore[attr-defined]

        target_elem: etree._Element | None = root.find(tag_path)  # type: ignore[attr-defined]

        if target_elem is not None:
            # Remove all children from target element
            for child in list(target_elem):  # type: ignore[arg-type]
                target_elem.remove(child)

            # Add indentation after opening tag
            target_elem.text = "\n    "

            for file_name in file_names:
                if file_name.exists():
                    with file_name.open() as f:
                        replacement_content = f.read()

                    # Add replacement content as XML elements
                    replacement_elems: etree._Element = etree.fromstring(f"<root>{replacement_content}</root>")  # type: ignore[attr-defined]
                    for elem in replacement_elems:  # type: ignore[attr-defined]
                        target_elem.append(elem)

        # Save changes to XML file
        tree.write(str(input_xml), pretty_print=True, xml_declaration=True, encoding="utf-8")

    except Exception as e:
        raise XMLGenerationError(f"Failed to modify XML configuration: {e}") from e


def generate_opnsense_config(
    csv_file: Path, base_config: Path, output_dir: Path, options: dict[str, Any] | None = None
) -> Path:
    """Generate complete OPNsense configuration from CSV data.

    This function integrates the functionality from the original generateXMLConfig.py
    from Stefan Reichhard's OPNsense configuration generator.

    Args:
        csv_file: Input CSV file with network configuration data
        base_config: Base OPNsense XML configuration file
        output_dir: Directory for generated files
        options: Configuration options

    Returns:
        Path to the generated configuration file

    Raises:
        XMLGenerationError: If XML generation fails
    """
    if options is None:
        options = {
            "firewallNr": 1,
            "opt_counter": 6,
            "wan1": "10.11.12.11",
            "wan2": "10.11.12.12",
            "wan3": "10.11.12.13",
        }

    # Configuration modules - derived from original MODULES definition
    modules: list[dict[str, Any]] = [
        {"order": 1, "part_name": "Interface", "tag_path": "./interfaces", "generator": generate_interface_xml},
        {"order": 2, "part_name": "DHCP", "tag_path": "./dhcpd", "generator": generate_dhcp_xml},
        {"order": 3, "part_name": "NAT", "tag_path": "./nat/outbound", "generator": generate_nat_xml},
        {"order": 4, "part_name": "Rules", "tag_path": "./filter", "generator": generate_rules_xml},
        {"order": 5, "part_name": "CARP", "tag_path": "./virtualip", "generator": generate_carp_xml},
        {"order": 6, "part_name": "VLAN", "tag_path": "./vlans", "generator": generate_vlan_xml},
        {
            "order": 7,
            "part_name": "RadiusUser",
            "tag_path": "./OPNsense/freeradius/user/users",
            "generator": generate_radius_user_xml,
        },
    ]

    try:
        # Ensure output directory exists
        output_dir.mkdir(parents=True, exist_ok=True)

        # Generate XML parts
        for module_info in modules:
            xml_file = output_dir / f"part{module_info['order']}_{module_info['part_name']}.xml"
            console.print(f"[blue]Generating {xml_file.name}...[/blue]")
            module_info["generator"](csv_file, xml_file, options)

        # Create final configuration
        final_config = output_dir / f"generated_{base_config.name}"
        shutil.copy2(base_config, final_config)

        # Inject generated parts into configuration
        for module_info in modules:
            xml_file = output_dir / f"part{module_info['order']}_{module_info['part_name']}.xml"
            if xml_file.exists():
                console.print(f"[blue]Injecting {xml_file.name} into configuration...[/blue]")
                modify_xml_config(final_config, module_info["tag_path"], [xml_file])

    except Exception as e:
        raise XMLGenerationError(f"Failed to generate OPNsense configuration: {e}") from e
    else:
        return final_config


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
    help="Generate realistic network configuration test data and OPNsense XML configurations",
    epilog="""Examples:

  CSV Generation:
    python generate_csv.py                    Generate 10 VLANs (default)
    python generate_csv.py --count 25         Generate 25 VLANs
    python generate_csv.py -c 50 -o my.csv   Generate 50 VLANs to my.csv

  OPNsense XML Generation:
    python generate_csv.py xml --base-config config.xml --csv-file my.csv
    python generate_csv.py xml --base-config config.xml --count 25

  Help:
    python generate_csv.py --help             Show this help message
    python generate_csv.py xml --help         Show XML generation help
    """,
    rich_markup_mode="rich",
)


@app.command("csv", help="Generate CSV file with network configuration data")
def generate_csv_command(
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
        Path | None,
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
    ] = None,
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
    # Set default output if not provided
    if output is None:
        output = DEFAULT_OUTPUT_PATH

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


@app.command("xml", help="Generate OPNsense XML configuration from CSV data")
def generate_xml_command(
    base_config: Annotated[
        Path,
        typer.Option(
            "--base-config",
            "-b",
            help="Base OPNsense XML configuration file to use as template",
            exists=True,
            file_okay=True,
            dir_okay=False,
            readable=True,
            resolve_path=True,
        ),
    ],
    csv_file: Annotated[
        Path | None,
        typer.Option(
            "--csv-file",
            "-c",
            help="CSV file with network configuration data (if not provided, will generate one)",
            exists=False,
            file_okay=True,
            dir_okay=False,
            resolve_path=True,
        ),
    ] = None,
    count: Annotated[
        int,
        typer.Option(
            "--count",
            "-n",
            help="Number of VLAN configurations to generate (used if --csv-file not provided)",
            min=1,
            max=MAX_VLAN_COUNT,
        ),
    ] = 10,
    output_dir: Annotated[
        Path | None,
        typer.Option(
            "--output-dir",
            "-o",
            help="Output directory for generated files",
            file_okay=False,
            dir_okay=True,
            writable=True,
            resolve_path=True,
        ),
    ] = None,
    firewall_nr: Annotated[
        int,
        typer.Option(
            "--firewall-nr",
            help="Firewall number (affects IP addressing)",
            min=1,
            max=253,
        ),
    ] = 1,
    opt_counter: Annotated[
        int,
        typer.Option(
            "--opt-counter",
            help="Starting OPT interface counter",
            min=1,
        ),
    ] = 6,
    force: Annotated[
        bool,
        typer.Option(
            "-f",
            "--force",
            help="Overwrite existing files without confirmation",
        ),
    ] = False,
) -> None:
    """Generate OPNsense XML configuration from CSV network data.

    Creates complete OPNsense configuration files including VLANs, interfaces,
    DHCP settings, and firewall rules based on CSV input data.
    """
    # Set default output_dir if not provided
    if output_dir is None:
        output_dir = Path("output")

    try:
        # Determine CSV file to use
        if csv_file is None:
            # Generate CSV file
            csv_file = output_dir / "generated-config.csv"
            console.print(f"[yellow]No CSV file provided, generating {count} configurations...[/yellow]")

            # Ensure output directory exists
            output_dir.mkdir(parents=True, exist_ok=True)

            generate_csv(csv_file, count)
            console.print(f"[green]✓ Generated CSV file: {csv_file}[/green]")

        # Validate CSV file
        if not csv_file.exists():
            stderr_console.print(f"[red]Error:[/red] CSV file not found: {csv_file}")
            raise typer.Exit(1)

        # Check if output directory exists and handle force option
        if output_dir.exists() and any(output_dir.iterdir()) and not force:
            console.print(f"[yellow]Output directory '{output_dir}' contains files.[/yellow]")
            if not Confirm.ask("Continue and potentially overwrite files", default=False):
                console.print("[blue]Operation cancelled.[/blue]")
                raise typer.Exit(0)

        # Prepare options
        options: dict[str, Any] = {
            "firewallNr": firewall_nr,
            "opt_counter": opt_counter,
            "wan1": "10.11.12.11",
            "wan2": "10.11.12.12",
            "wan3": "10.11.12.13",
        }

        # Display generation info
        console.print("[green]Generating OPNsense configuration...[/green]")
        console.print(f"[blue]Base config:[/blue] {base_config}")
        console.print(f"[blue]CSV file:[/blue] {csv_file}")
        console.print(f"[blue]Output directory:[/blue] {output_dir}")
        console.print(f"[blue]Firewall number:[/blue] {firewall_nr}")
        console.print(f"[blue]Starting OPT counter:[/blue] {opt_counter}")

        # Generate the OPNsense configuration
        final_config = generate_opnsense_config(csv_file, base_config, output_dir, options)

        # Success message
        console.print(f"[green]✓ Successfully generated OPNsense configuration: {final_config}[/green]")
        console.print(f"[green]✓ Generated XML parts are available in: {output_dir}[/green]")
        console.print("[yellow]Note:[/yellow] This functionality is derived from Stefan Reichhard's original work")

    except XMLGenerationError as e:
        stderr_console.print(f"[red]XML Generation Error:[/red] {e}")
        raise typer.Exit(1) from e
    except ConfigGenerationError as e:
        stderr_console.print(f"[red]CSV Generation Error:[/red] {e}")
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


# Set CSV generation as the default command
@app.callback(invoke_without_command=True)
def main(
    ctx: typer.Context,
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
        Path | None,
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
    ] = None,
    force: Annotated[
        bool,
        typer.Option(
            "-f",
            "--force",
            help="Overwrite existing output file without confirmation",
        ),
    ] = False,
) -> None:
    """Generate network configuration data (CSV by default, XML with 'xml' command)."""
    if ctx.invoked_subcommand is None:
        # Default behavior - generate CSV
        # Set default output if not provided
        if output is None:
            output = DEFAULT_OUTPUT_PATH
        generate_csv_command(count, output, force)


if __name__ == "__main__":
    app()
