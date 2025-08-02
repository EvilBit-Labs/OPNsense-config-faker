# Python Development Standards for OPNsense Config Faker

## 1. Python Version

- Use Python 3.10+ for all development
- Primary development on Python 3.13
- Ensure compatibility with Python 3.10+ in CI/CD

## 2. Code Style

- Follow PEP 8 style guidelines
- Use `black` for automatic code formatting
- Line length: 119 characters (as per project standards)
- Use type hints for all function parameters and return values

## 3. Dependencies

- Manage dependencies with `pip` and `requirements.txt`
- Pin exact versions in `requirements.txt`
- Use virtual environments for isolation
- Primary dependencies:
  - `faker` for realistic test data generation
  - `lxml` for XML processing (legacy compatibility)
  - `tzdata` for timezone support

## 4. Code Organization

- Keep main functionality in `generate_csv.py`
- Use clear function and variable names
- Add docstrings to all functions and classes
- Separate concerns: data generation, file I/O, CLI handling

## 5. Error Handling

- Use appropriate exception types
- Provide meaningful error messages
- Handle edge cases gracefully
- Log errors appropriately

## 6. Data Generation

- Use `faker` for all synthetic data generation
- Ensure generated data is realistic and valid
- Follow RFC standards for network configurations:
  - RFC 1918 for private IP addresses
  - Valid VLAN ID ranges (10-4094)
  - Realistic department/organizational naming

## 7. File Operations

- Use `pathlib` for path operations when possible
- Create directories automatically if they don't exist
- Handle file permissions appropriately
- Use context managers for file operations

## 8. Command Line Interface

- Use `argparse` for CLI argument parsing
- Provide helpful error messages and usage information
- Support both short and long argument forms
- Include comprehensive examples in help text
