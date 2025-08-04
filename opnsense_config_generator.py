"""OPNsense Config Generator - Direct XML Configuration Generator.

Generates valid OPNsense config.xml files with realistic faked data.
Designed specifically for testing OPNsense automation tools and infrastructure.

This module contains functionality derived from or inspired by the original
OPNsense configuration generator by Stefan Reichhard (nett-media).

Original work: https://github.com/nett-media/opnsense-config-generator
Original author: Stefan Reichhard
Initial implementation: November 2023

Enhanced and modernized by EvilBit Labs for direct OPNsense configuration generation.

This implementation maintains the core concepts while adding:
- Modern Python practices and type hints
- Faker integration for realistic test data
- Direct in-memory data generation (no CSV intermediate)
- Improved error handling and validation
- Modular architecture for extensibility
"""

from __future__ import annotations

import ipaddress
import random
import shutil
import string
import sys
import time
import uuid
from dataclasses import dataclass
from pathlib import Path
from typing import Annotated, Any
from xml.sax.saxutils import escape

import typer
from faker import Faker
from rich.console import Console
from rich.progress import Progress, SpinnerColumn, TextColumn
from rich.prompt import Confirm

try:
    from lxml import etree  # type: ignore[attr-defined]

    lxml_available = True
except ImportError:
    etree = None  # type: ignore[assignment]
    lxml_available = False

# Initialize global instances
fake = Faker()
console = Console()
stderr_console = Console(file=sys.stderr)

# Constants
MAX_VLAN_COUNT = 4084  # Maximum practical VLAN count (4094 - 10 reserved VLANs)
WAN1_ID = 1
WAN2_ID = 2
WAN3_ID = 3


@dataclass
class VLANConfig:
    """VLAN configuration data structure."""

    vlan_id: int
    ip_network: str  # e.g., "10.123.45.x"
    description: str
    wan_assignment: int  # 1-3


class ConfigGenerationError(Exception):
    """Raised when configuration generation fails."""


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


def generate_vlan_configurations(count: int) -> list[VLANConfig]:
    """Generate VLAN configuration data structures.

    Args:
        count: Number of VLAN configurations to generate

    Returns:
        List of VLANConfig objects

    Raises:
        ConfigGenerationError: If generation fails
        ValueError: If count is invalid
    """
    if count < 1:
        raise ValueError("Number of configurations must be at least 1")

    if count > MAX_VLAN_COUNT:
        stderr_console.print(
            f"[yellow]Warning:[/yellow] Requested count ({count}) exceeds practical VLAN limit "
            f"({MAX_VLAN_COUNT}), may have duplicate issues"
        )

    try:
        configs: list[VLANConfig] = []
        used_vlans: set[int] = set()
        used_networks: set[str] = set()

        with Progress(
            SpinnerColumn(),
            TextColumn("[progress.description]{task.description}"),
            console=console,
            transient=True,
        ) as progress:
            task = progress.add_task("Generating VLAN configurations...", total=count)

            for _ in range(count):
                # Generate unique VLAN ID
                while True:
                    vlan_id = fake.random_int(min=10, max=4094)  # Valid VLAN range
                    if vlan_id not in used_vlans:
                        used_vlans.add(vlan_id)
                        break

                # Generate a unique private IPv4 network
                while True:
                    private_ip = fake.ipv4_private()
                    ip_obj = ipaddress.IPv4Address(private_ip)
                    octets = str(ip_obj).split(".")
                    network_base = f"{octets[0]}.{octets[1]}.{octets[2]}.x"

                    if network_base not in used_networks:
                        used_networks.add(network_base)
                        break

                # Generate realistic description
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
                description = f"{department}{vlan_id}"

                # WAN assignment (1-3)
                wan_assignment = fake.random_int(min=1, max=3)

                configs.append(
                    VLANConfig(
                        vlan_id=vlan_id,
                        ip_network=network_base,
                        description=description,
                        wan_assignment=wan_assignment,
                    )
                )

                progress.advance(task)

    except Exception as e:
        raise ConfigGenerationError(f"Failed to generate VLAN configurations: {e}") from e
    else:
        return configs


def generate_vlan_xml(configs: list[VLANConfig], output_file: Path) -> None:
    """Generate VLAN XML configuration from VLAN configurations.

    This function is derived from the original genVLAN.py
    from Stefan Reichhard's OPNsense configuration generator.

    Args:
        configs: List of VLAN configurations
        output_file: Output XML file path
    """
    with output_file.open("w") as outfile:
        for config in configs:
            vlan_uuid = str(uuid.uuid4())

            outfile.write(f'    <vlan uuid="{vlan_uuid}">\n')
            outfile.write("      <if>lagg0</if>\n")
            outfile.write(f"      <tag>{config.vlan_id}</tag>\n")
            outfile.write("      <pcp>0</pcp>\n")
            outfile.write("      <proto/>\n")
            outfile.write(f"      <descr>{config.description}</descr>\n")
            outfile.write(f"      <vlanif>vlan0{config.vlan_id}</vlanif>\n")
            outfile.write("    </vlan>\n")


def generate_interface_xml(configs: list[VLANConfig], output_file: Path, options: dict[str, Any]) -> None:
    """Generate Interface XML configuration from VLAN configurations.

    This function is derived from the original genInterface.py
    from Stefan Reichhard's OPNsense configuration generator.

    Args:
        configs: List of VLAN configurations
        output_file: Output XML file path
        options: Configuration options
    """
    opt_counter: int = options.get("opt_counter", 1)
    firewall_number: int = options.get("firewallNr", 1)
    ip_suffix: int = 250 + firewall_number

    with output_file.open("w") as outfile:
        for config in configs:
            ip_address = config.ip_network.replace(".x", f".{ip_suffix}")
            description = escape_xml_string(config.description)

            outfile.write(f"<opt{opt_counter}>\n")
            outfile.write(f"  <if>vlan0{config.vlan_id}</if>\n")
            outfile.write(f"  <descr>V{config.vlan_id}_{description}</descr>\n")
            outfile.write("  <enable>1</enable>\n")
            outfile.write("  <spoofmac/>\n")
            outfile.write(f"  <ipaddr>{ip_address}</ipaddr>\n")
            outfile.write("  <subnet>24</subnet>\n")
            outfile.write(f"</opt{opt_counter}>\n")

            opt_counter += 1


def generate_dhcp_xml(configs: list[VLANConfig], output_file: Path, options: dict[str, Any]) -> None:
    """Generate DHCP XML configuration from VLAN configurations.

    This function is derived from the original genDHCP.py
    from Stefan Reichhard's OPNsense configuration generator.

    Args:
        configs: List of VLAN configurations
        output_file: Output XML file path
        options: Configuration options
    """
    opt_counter: int = options.get("opt_counter", 1)

    with output_file.open("w") as outfile:
        for config in configs:
            ip_range_base = config.ip_network.replace(".x", "")

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


def generate_rules_xml(configs: list[VLANConfig], output_file: Path, options: dict[str, Any]) -> None:
    """Generate firewall rules XML configuration from VLAN configurations.

    This function is derived from the original genRules.py
    from Stefan Reichhard's OPNsense configuration generator.

    Args:
        configs: List of VLAN configurations
        output_file: Output XML file path
        options: Configuration options
    """
    opt_counter: int = options.get("opt_counter", 1)

    with output_file.open("w") as outfile:
        for config in configs:
            random_uuid = str(uuid.uuid4())
            timestamp = time.time()
            formatted_timestamp = f"{timestamp:.4f}"

            outfile.write(f'<rule uuid="{random_uuid}">\n')
            outfile.write("  <type>pass</type>\n")
            outfile.write(f"  <interface>opt{opt_counter}</interface>\n")
            outfile.write("  <ipprotocol>inet</ipprotocol>\n")
            outfile.write("  <statetype>keep state</statetype>\n")
            outfile.write(f"  <descr>default allow VLAN_{config.vlan_id} any</descr>\n")
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
            outfile.write("    <description>OPNsense Config Generator</description>\n")
            outfile.write("  </updated>\n")
            outfile.write("  <created>\n")
            outfile.write("    <username>root@10.1.1.1</username>\n")
            outfile.write(f"    <time>{formatted_timestamp}</time>\n")
            outfile.write("    <description>OPNsense Config Generator</description>\n")
            outfile.write("  </created>\n")
            outfile.write("</rule>\n\n")

            opt_counter += 1


def generate_nat_xml(configs: list[VLANConfig], output_file: Path, options: dict[str, Any]) -> None:
    """Generate NAT XML configuration from VLAN configurations.

    This function is derived from the original genNAT.py
    from Stefan Reichhard's OPNsense configuration generator.

    Args:
        configs: List of VLAN configurations
        output_file: Output XML file path
        options: Configuration options
    """
    opt_counter: int = options.get("opt_counter", 1)

    with output_file.open("w") as outfile:
        for config in configs:
            timestamp = time.time()
            formatted_timestamp = f"{timestamp:.4f}"

            wan_ip: str = ""
            if config.wan_assignment == WAN1_ID:
                wan_ip = options.get("wan1", "80.200.10.11")
            elif config.wan_assignment == WAN2_ID:
                wan_ip = options.get("wan2", "80.200.10.12")
            elif config.wan_assignment == WAN3_ID:
                wan_ip = options.get("wan3", "80.200.10.13")

            outfile.write("<rule>\n")
            outfile.write("  <source>\n")
            outfile.write(f"    <network>opt{opt_counter}</network>\n")
            outfile.write("  </source>\n")
            outfile.write("  <destination>\n")
            outfile.write("    <any>1</any>\n")
            outfile.write("  </destination>\n")
            outfile.write(f"  <descr>{config.description}</descr>\n")
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
            outfile.write("    <description>OPNsense Config Generator</description>\n")
            outfile.write("  </created>\n")
            outfile.write(f"  <target>{wan_ip}</target>\n")
            outfile.write("  <sourceport/>\n")
            outfile.write("  <updated>\n")
            outfile.write("    <username>root@10.1.1.1</username>\n")
            outfile.write(f"    <time>{formatted_timestamp}</time>\n")
            outfile.write("    <description>OPNsense Config Generator</description>\n")
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


def generate_carp_xml(configs: list[VLANConfig], output_file: Path, options: dict[str, Any]) -> None:
    """Generate CARP virtual IP XML configuration from VLAN configurations.

    This function is derived from the original genCARP.py
    from Stefan Reichhard's OPNsense configuration generator.

    Args:
        configs: List of VLAN configurations
        output_file: Output XML file path
        options: Configuration options
    """
    opt_counter: int = options.get("opt_counter", 1)
    firewall_number: int = options.get("firewallNr", 1)
    advskew = "0" if firewall_number == 1 else "100"

    with output_file.open("w") as outfile:
        for config in configs:
            ip_range = config.ip_network.replace(".x", ".254")
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
            outfile.write(f"    <vhid>{config.vlan_id}</vhid>\n")
            outfile.write("    <advbase>1</advbase>\n")
            outfile.write(f"    <advskew>{advskew}</advskew>\n")
            outfile.write(f"    <descr>{config.description}</descr>\n")
            outfile.write("  </vip>\n")

            opt_counter += 1


def generate_radius_user_xml(configs: list[VLANConfig], output_file: Path) -> None:
    """Generate RADIUS user XML configuration from VLAN configurations.

    This function is derived from the original genRadiusUser.py
    from Stefan Reichhard's OPNsense configuration generator.

    Args:
        configs: List of VLAN configurations
        output_file: Output XML file path
    """
    with output_file.open("w") as outfile:
        for config in configs:
            user_uuid = str(uuid.uuid4())
            username = f"top{config.vlan_id}"
            password = escape_xml_string(config.description)

            outfile.write(f'        <user uuid="{user_uuid}">\n')
            outfile.write("          <enabled>1</enabled>\n")
            outfile.write(f"          <username>{username}</username>\n")
            outfile.write(f"          <password>{password}</password>\n")
            outfile.write(f"          <description>{config.description}</description>\n")
            outfile.write("          <ip/>\n")
            outfile.write("          <subnet/>\n")
            outfile.write("          <route/>\n")
            outfile.write("          <ip6/>\n")
            outfile.write(f"          <vlan>{config.vlan_id}</vlan>\n")
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
        XMLGenerationError: If lxml is not available or XML processing fails
    """
    if not lxml_available:
        raise XMLGenerationError("lxml is required for XML configuration generation. Install with: uv add lxml")

    try:
        parser: etree.XMLParser = etree.XMLParser(remove_blank_text=True)  # type: ignore[attr-defined,union-attr]
        tree: etree._ElementTree = etree.parse(str(input_xml), parser)  # type: ignore[attr-defined,union-attr]
        root: etree._Element = tree.getroot()  # type: ignore[attr-defined,union-attr]

        target_elem: etree._Element | None = root.find(tag_path)  # type: ignore[attr-defined,union-attr]

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
                    replacement_elems: etree._Element = etree.fromstring(f"<root>{replacement_content}</root>")  # type: ignore[attr-defined,union-attr]
                    for elem in replacement_elems:  # type: ignore[attr-defined]
                        target_elem.append(elem)

        # Save changes to XML file
        tree.write(str(input_xml), pretty_print=True, xml_declaration=True, encoding="utf-8")

    except Exception as e:
        raise XMLGenerationError(f"Failed to modify XML configuration: {e}") from e


def generate_opnsense_config(
    base_config: Path, output_dir: Path, count: int, options: dict[str, Any] | None = None
) -> Path:
    """Generate complete OPNsense configuration with realistic faked data.

    This function integrates the functionality from the original generateXMLConfig.py
    from Stefan Reichhard's OPNsense configuration generator.

    Args:
        base_config: Base OPNsense XML configuration file
        output_dir: Directory for generated files
        count: Number of VLAN configurations to generate
        options: Configuration options

    Returns:
        Path to the generated configuration file

    Raises:
        XMLGenerationError: If XML generation fails
        ConfigGenerationError: If configuration generation fails
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
        # Generate VLAN configurations
        console.print(f"[green]Generating {count} VLAN configurations...[/green]")
        vlan_configs = generate_vlan_configurations(count)

        # Ensure output directory exists
        output_dir.mkdir(parents=True, exist_ok=True)

        # Generate XML parts
        for module_info in modules:
            xml_file = output_dir / f"part{module_info['order']}_{module_info['part_name']}.xml"
            console.print(f"[blue]Generating {xml_file.name}...[/blue]")

            # Call the appropriate generator function
            if module_info["part_name"] in ("RadiusUser", "VLAN"):
                module_info["generator"](vlan_configs, xml_file)
            else:
                module_info["generator"](vlan_configs, xml_file, options)

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


# Initialize Typer app
app = typer.Typer(
    name="opnsense-config-generator",
    help="Generate valid OPNsense config.xml files with realistic faked data",
    epilog="""Examples:

  Generate OPNsense configuration:
    python opnsense_config_generator.py --base-config config.xml --count 25
    python opnsense_config_generator.py -b config.xml -c 50 -o /path/to/output

  Help:
    python opnsense_config_generator.py --help
    """,
    rich_markup_mode="rich",
)


@app.command(help="Generate OPNsense XML configuration with realistic faked data")
def main(
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
    count: Annotated[
        int,
        typer.Option(
            "--count",
            "-c",
            help="Number of VLAN configurations to generate",
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
    """Generate OPNsense XML configuration with realistic faked data.

    Creates complete OPNsense configuration files including VLANs, interfaces,
    DHCP settings, NAT rules, firewall rules, CARP virtual IPs, and RADIUS users
    based on generated realistic network data.

    Requires lxml package for XML processing: uv add lxml
    """
    # Set default output_dir if not provided
    if output_dir is None:
        output_dir = Path("output")

    try:
        # Check if lxml is available
        if not lxml_available:
            stderr_console.print(
                "[red]Error:[/red] lxml is required for XML generation.\n[blue]Install with:[/blue] uv add lxml"
            )
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
        console.print(f"[blue]Output directory:[/blue] {output_dir}")
        console.print(f"[blue]VLAN count:[/blue] {count}")
        console.print(f"[blue]Firewall number:[/blue] {firewall_nr}")
        console.print(f"[blue]Starting OPT counter:[/blue] {opt_counter}")

        # Generate the OPNsense configuration
        final_config = generate_opnsense_config(base_config, output_dir, count, options)

        # Success message
        console.print(f"[green]✓ Successfully generated OPNsense configuration: {final_config}[/green]")
        console.print(f"[green]✓ Generated XML parts are available in: {output_dir}[/green]")
        console.print("[yellow]Note:[/yellow] This functionality is derived from Stefan Reichhard's original work")

    except XMLGenerationError as e:
        stderr_console.print(f"[red]XML Generation Error:[/red] {e}")
        raise typer.Exit(1) from e
    except ConfigGenerationError as e:
        stderr_console.print(f"[red]Configuration Generation Error:[/red] {e}")
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
