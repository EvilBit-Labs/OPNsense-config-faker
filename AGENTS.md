# OPNsense Config Faker - AI Coding Assistant Rules

This document outlines the core concepts, framework, and coding standards for the OPNsense Config Faker project. It serves as a comprehensive guide for AI coding assistants to ensure consistency, maintaibility, and adherence to established best practices.

## 1. Core Philosophy

- **Data Generation Focus**: This project specializes in generating realistic network configuration test data using the Faker library
- **Operator-Centric Design**: Built for network operators and automation engineers who need realistic test data
- **Ethical Development**: Maintain proper boundaries and respect for the original nett-media/opnsense-config-generator project
- **Structured Data**: All generated configurations should be structured, consistent, and useful for testing purposes

## 2. Project Structure

```text
/
├── generate_csv.py           # Main CSV generation script
├── run_generator.sh          # Interactive helper script
├── legacy/                   # Original OPNsense functionality (preserved)
├── output/                   # Generated configuration files
├── tests/                    # Test suite (when implemented)
├── justfile                  # Task runner configuration
└── pyproject.toml           # Project metadata and dependencies (uv)
```

## 3. Technology Stack

| Layer               | Technology                      | Notes                                    |
| ------------------- | ------------------------------- | ---------------------------------------- |
| **Language**        | Python 3.10+                    | Modern Python with type hints            |
| **Data Generation** | Faker                           | For realistic network configuration data |
| **XML Processing**  | lxml                            | For legacy XML functionality             |
| **CLI Enhancement** | Rich (potential future)         | For enhanced terminal output             |
| **Testing**         | pytest                          | When test suite is implemented           |
| **Tooling**         | `uv` for deps, `just` for tasks | `ruff` for linting and formatting        |

## 4. Coding Standards and Conventions

### Python

- **Formatting**: `ruff format` with a line length of 119 characters
- **Linting**: `ruff` to enforce style and catch errors
- **Type Hinting**: Mandatory for all functions and methods. Use modern union types (`str | None`)
- **Naming**: `snake_case` for variables and functions, `CamelCase` for classes
- **Error Handling**: Raise specific, descriptive exceptions with proper context
- **Dependencies**: Manage with `uv`. No `requirements.txt`
- **Documentation**: Clear docstrings for all public functions

### Data Generation Principles

- **Realistic Data**: Generate RFC-compliant network configurations
- **Unique Values**: Ensure no duplicate VLAN IDs or conflicting IP ranges
- **Configurable**: Allow users to specify count, output format, and other parameters
- **Consistent**: Maintain consistent data patterns across generated records

### Commit Messages

- **Conventional Commits**: All commit messages must adhere to the [Conventional Commits](https://www.conventionalcommits.org) specification
  - **Types**: `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `build`, `ci`, `chore`
  - **Scopes**: `(cli)`, `(data)`, `(legacy)`, `(docs)`, etc.
  - **Breaking Changes**: Indicated with `!` in the header or `BREAKING CHANGE:` in the footer

## 5. Development Guidelines

### Core Functionality

- **Main Script**: `generate_csv.py` - Generates CSV files with network configurations
- **Default Output**: `output/test-config.csv`
- **CLI Interface**: Supports `--count` and `--output` parameters
- **Data Format**: CSV with columns: VLAN, IP Range, Beschreibung, WAN

### Legacy Support

- **Preservation**: Original OPNsense functionality preserved in `legacy/` directory
- **Guidance**: Users encouraged to use upstream project for OPNsense-specific features
- **Separation**: Clear distinction between new data generation features and legacy code

### Quality Assurance

1. **Code Quality**: All code must pass `ruff` linting and formatting checks
2. **Type Safety**: Comprehensive type hints for all functions
3. **Testing**: Generate valid configurations that can be used in real testing scenarios
4. **Documentation**: Clear documentation for all user-facing functionality
5. **Backwards Compatibility**: Maintain compatibility with existing usage patterns

## 6. AI Assistant Behavior

- **Clarity and Precision**: Be direct, professional, and context-aware in all interactions
- **Adherence to Standards**: Strictly follow the defined rules for code style and project structure
- **Tool Usage**: Use `uv` for dependency management and `just` for task execution
- **Respect Legacy**: Don't modify legacy code unless specifically requested
- **Focus on Value**: Enhance the project's unique value proposition as a network configuration data generator
- **Ethical Boundaries**: Always respect the original project authors and encourage upstream usage where appropriate

## 7. Future Roadmap Considerations

- Enhanced CLI with Rich for better user experience
- Comprehensive test suite with pytest
- Additional network configuration elements (firewall rules, DHCP scopes, etc.)
- Configuration validation and consistency checking
- Export formats beyond CSV (JSON, YAML, etc.)
