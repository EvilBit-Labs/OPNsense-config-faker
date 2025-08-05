#!/bin/bash
# Network Configuration Data Generator Setup Script

set -e

echo "=== Network Configuration Data Generator Setup ==="
echo

# Check for Python 3.10+
if command -v python3.13 &> /dev/null; then
    PYTHON_CMD="python3.13"
elif command -v python3.12 &> /dev/null; then
    PYTHON_CMD="python3.12"
elif command -v python3.11 &> /dev/null; then
    PYTHON_CMD="python3.11"
elif command -v python3.10 &> /dev/null; then
    PYTHON_CMD="python3.10"
elif command -v python3 &> /dev/null; then
    PYTHON_VERSION=$(python3 --version | cut -d' ' -f2 | cut -d'.' -f1-2)
    if [[ $(echo "$PYTHON_VERSION >= 3.10" | bc -l) -eq 1 ]]; then
        PYTHON_CMD="python3"
    else
        echo "Error: Python 3.10+ is required. Found Python $PYTHON_VERSION"
        exit 1
    fi
else
    echo "Error: Python 3.10+ is required but not found."
    echo "Please install Python 3.10+ and try again."
    exit 1
fi

echo "Using Python: $($PYTHON_CMD --version)"

# Create virtual environment
echo "Creating virtual environment..."
$PYTHON_CMD -m venv venv

# Activate virtual environment
echo "Activating virtual environment..."
if [ ! -f "venv/bin/activate" ]; then
    echo "Error: Virtual environment activation script not found at venv/bin/activate"
    echo "The virtual environment may not have been created properly."
    exit 1
fi
# shellcheck source=/dev/null
source venv/bin/activate

# Upgrade pip
echo "Upgrading pip..."
pip install --upgrade pip

# Install dependencies
echo "Installing dependencies..."
pip install -r requirements.txt

echo
echo "✅ Setup complete!"
echo
echo "To use the network configuration data generator:"
echo "1. Activate the virtual environment: source venv/bin/activate"
echo "2. Generate data: python generate_csv.py"
echo "3. Or use the interactive script: ./run_generator.sh"
echo
echo "For help: python generate_csv.py --help"
