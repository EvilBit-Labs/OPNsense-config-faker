#!/bin/bash
# OPNsense Config Generator Helper Script

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}Network Configuration Data Generator${NC}"
echo "===================================="

# Check if virtual environment exists
if [ ! -d "venv" ]; then
    echo -e "${YELLOW}Virtual environment not found. Creating...${NC}"
    /opt/homebrew/bin/python3.13 -m venv venv
    source venv/bin/activate
    pip install -r requirements.txt
else
    echo -e "${GREEN}Activating virtual environment...${NC}"
    source venv/bin/activate
fi

# Check Python version
python_version=$(python --version | cut -d' ' -f2)
echo "Using Python $python_version"

# Create output directory if it doesn't exist
mkdir -p output

echo -e "${GREEN}Available commands:${NC}"
echo "1. Generate network configuration data (CSV)"
echo "2. Generate with custom count"
echo "3. Generate with custom filename"
echo "4. Legacy: Use original OPNsense generator"

read -p "Choose an option (1-4): " choice

case $choice in
    1)
        echo -e "${YELLOW}Generating network configuration data...${NC}"
        python generate_csv.py
        ;;
    2)
        read -p "How many VLAN configurations to generate? (default: 10): " vlan_count
        echo -e "${YELLOW}Generating $vlan_count VLAN configurations...${NC}"
        if [ -z "$vlan_count" ]; then
            python generate_csv.py
        else
            python generate_csv.py --count "$vlan_count"
        fi
        ;;
    3)
        read -p "Output filename (default: test-config.csv): " filename
        read -p "How many VLAN configurations? (default: 10): " vlan_count
        echo -e "${YELLOW}Generating network configuration data...${NC}"
        cmd="python generate_csv.py"
        if [ ! -z "$vlan_count" ]; then
            cmd="$cmd --count $vlan_count"
        fi
        if [ ! -z "$filename" ]; then
            cmd="$cmd --output $filename"
        fi
        eval $cmd
        ;;
    4)
        echo -e "${YELLOW}Legacy OPNsense generator...${NC}"
        echo -e "${YELLOW}For OPNsense XML generation, we recommend using the upstream project:${NC}"
        echo -e "${GREEN}https://github.com/nett-media/opnsense-config-generator${NC}"
        echo
        echo -e "${YELLOW}This ensures you get the latest updates and support from the original authors.${NC}"
        echo
        read -p "Do you still want to use the legacy version? (y/N): " use_legacy
        if [[ $use_legacy =~ ^[Yy]$ ]]; then
            if [ -f "legacy/opnsense/generateXMLConfig.py" ]; then
                cd legacy/opnsense
                python generateXMLConfig.py
                cd ../..
                echo -e "${GREEN}Generated XML files in legacy/opnsense/export/ directory${NC}"
            else
                echo -e "${RED}Legacy OPNsense generator not found${NC}"
                exit 1
            fi
        else
            echo -e "${GREEN}Consider cloning the upstream project for OPNsense functionality.${NC}"
        fi
        ;;
    *)
        echo -e "${RED}Invalid option${NC}"
        exit 1
        ;;
esac

echo -e "${GREEN}Done!${NC}"
