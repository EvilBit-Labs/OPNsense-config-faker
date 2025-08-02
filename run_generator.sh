#!/bin/bash
# OPNsense Config Generator Helper Script

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}OPNsense Config Generator${NC}"
echo "=========================="

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

# Create export directory if it doesn't exist
mkdir -p export

echo -e "${GREEN}Available commands:${NC}"
echo "1. Generate new test CSV file"
echo "2. Run XML config generator"
echo "3. Both (generate CSV then XML)"

read -p "Choose an option (1-3): " choice

case $choice in
    1)
        echo -e "${YELLOW}Generating new test CSV file...${NC}"
        read -p "How many VLANs to generate? (default: 10): " vlan_count
        if [ -z "$vlan_count" ]; then
            python generate_csv.py
        else
            python generate_csv.py --count "$vlan_count"
        fi
        ;;
    2)
        echo -e "${YELLOW}Running XML config generator...${NC}"
        python generateXMLConfig.py
        echo -e "${GREEN}Generated XML files in export/ directory${NC}"
        ;;
    3)
        echo -e "${YELLOW}Generating new test CSV file...${NC}"
        read -p "How many VLANs to generate? (default: 10): " vlan_count
        if [ -z "$vlan_count" ]; then
            python generate_csv.py
        else
            python generate_csv.py --count "$vlan_count"
        fi
        echo -e "${YELLOW}Running XML config generator...${NC}"
        python generateXMLConfig.py
        echo -e "${GREEN}Generated XML files in export/ directory${NC}"
        ;;
    *)
        echo -e "${RED}Invalid option${NC}"
        exit 1
        ;;
esac

echo -e "${GREEN}Done!${NC}"
echo "Generated files are in the export/ directory"
