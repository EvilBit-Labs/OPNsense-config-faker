# Documentation Standards for OPNsense Config Faker

## 1. General Guidelines

- Use clear, concise language
- Include practical examples for all features
- Update documentation with every code change
- Maintain consistency in tone and style

## 2. README Structure

- **Project Title and Description**: Clear explanation of purpose
- **Origin and Attribution**: Proper credit to original project
- **Quick Start**: Get users running in under 5 minutes
- **Installation**: Comprehensive setup instructions
- **Usage**: Command-line examples and options
- **Data Format**: Explanation of generated data structure
- **Customization**: How to modify and extend
- **Roadmap**: Future development plans

## 3. Code Documentation

### Docstrings
- Use triple quotes for all docstrings
- Include parameter descriptions and types
- Document return values and exceptions
- Example:
```python
def generate_csv(filename: str, num_records: int) -> None:
    """Generate CSV file with network configuration data.
    
    Args:
        filename: Output CSV file path
        num_records: Number of VLAN configurations to generate
    
    Raises:
        IOError: If file cannot be created
        ValueError: If num_records is less than 1
    """
```

### Comments
- Explain complex logic and business rules
- Document assumptions and limitations
- Use TODO/FIXME/NOTE as appropriate

## 4. User-Facing Documentation

### Command Line Help
- Clear descriptions for all arguments
- Include practical examples
- Show expected output formats
- Explain default behaviors

### Error Messages
- Provide actionable guidance
- Include specific details about the problem
- Suggest possible solutions

## 5. Project Documentation Files

### AGENTS.md
- Core concepts and framework
- Guidelines for AI assistants
- Project structure overview

### .cursor/rules/
- Specific development standards
- Code style guidelines
- Testing requirements
- Contribution processes

## 6. Legacy Documentation

- Clearly explain relationship to upstream project
- Guide users to appropriate resources
- Maintain boundaries between projects
- Preserve attribution and licensing information

## 7. Maintenance

- Review documentation quarterly
- Update for new features and changes
- Remove outdated information
- Verify all links and references work correctly
