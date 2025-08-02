# AGENTS.md

This document outlines the core concepts and framework of the OPNsense Config Faker project. It serves as a guide for AI coding assistants to ensure consistency, maintainability, and adherence to the project's goals.

## Core Concepts

- **Purpose**: Generate realistic network configuration test data using Faker
- **Focus**: Data generation for testing network automation tools
- **Original Codebase**: Forked from nett-media/opnsense-config-generator
- **Ethics**: Maintain proper boundaries and respect original work

## Framework

- **Language**: Python 3.10+
- **Dependencies**:
  - `faker` for test data generation
  - `lxml` for XML processing
  - `tzdata` for timezone information

## Structure

- **Main Script**: `generate_csv.py`
  - Description: Generates CSV files with network configurations
  - Default Output: `output/test-config.csv`

- **Interactive Script**: `run_generator.sh`
  - Provides an interactive user interface

- **Legacy OPNsense Functionality**:
  - Preserved in `legacy/` directory
  - Users are encouraged to use the upstream project

## Guidelines

1. **Respect Original Authors**: Encourage using the upstream project for OPNsense-specific features
2. **Focus on Data Generation**: Enhance the project's unique value
3. **Clean Structure**: Keep generated files in `output/`
4. **Documentation**: Maintain clear and comprehensive documentation
5. **Testing**: Ensure all generated configurations are valid and useful for testing
