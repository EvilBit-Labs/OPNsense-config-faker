from faker import Faker
import csv
import ipaddress
import argparse
import sys
import os

fake = Faker()


def generate_csv(filename, num_records):
    with open(filename, mode="w", newline="") as file:
        writer = csv.writer(file)
        writer.writerow(["VLAN", "IP Range", "Beschreibung", "WAN"])

        # Keep track of used VLANs and networks to avoid duplicates
        used_vlans = set()
        used_networks = set()

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


def main():
    parser = argparse.ArgumentParser(
        description="Generate test CSV configuration files for OPNsense config generator",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""Examples:
  python generate_csv.py                    # Generate 10 VLANs (default)
  python generate_csv.py --count 25         # Generate 25 VLANs
  python generate_csv.py -c 50 -o my.csv   # Generate 50 VLANs to my.csv
        """,
    )

    parser.add_argument(
        "-c",
        "--count",
        type=int,
        default=10,
        help="Number of VLAN configurations to generate (default: 10)",
    )

    parser.add_argument(
        "-o", "--output",
        type=str,
        default="output/test-config.csv",
        help="Output CSV filename (default: output/test-config.csv)"
    )

    args = parser.parse_args()

    # Validate count
    if args.count < 1:
        print("Error: Count must be at least 1", file=sys.stderr)
        sys.exit(1)

    if args.count > 4084:  # Max VLANs available (4094 - 10)
        print(
            "Warning: Requested count exceeds practical VLAN limit, may have duplicate issues"
        )

    # Ensure output directory exists
    output_dir = os.path.dirname(args.output)
    if output_dir and not os.path.exists(output_dir):
        os.makedirs(output_dir)

    print(f"Generating {args.count} VLAN configurations...")
    print(f"Output file: {args.output}")

    try:
        generate_csv(args.output, args.count)
        print(
            f"Successfully generated {args.count} VLAN configurations in {args.output}"
        )
    except Exception as e:
        print(f"Error generating CSV: {e}", file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()
