# Security

This document covers security considerations, best practices, and policies for the OPNsense Config Faker project.

## Security Policy

### Supported Versions

| Version | Supported |
| ------- | --------- |
| 0.2.x   | Yes       |
| 0.1.x   | No        |
| < 0.1   | No        |

## Reporting Vulnerabilities

### For Security Researchers and Users

**IMPORTANT**: This is a one-person operation. Please be patient and understanding with response times.

#### How to Report

1. **DO** create a private GitHub security advisory for vulnerabilities
2. **DO NOT** post about the vulnerability in public forums or social media
3. **DO** use GitHub's security advisory feature for responsible disclosure
4. **DO** include "SECURITY VULNERABILITY" in the advisory title

#### What to Include in Your Report

Please provide as much detail as possible:

- **Description**: Clear explanation of the vulnerability
- **Impact**: What could an attacker do with this vulnerability?
- **Steps to Reproduce**: Detailed steps to demonstrate the issue
- **Environment**: OS, Rust version, dependencies, etc.
- **Proof of Concept**: Code or commands that demonstrate the issue (if safe to share)
- **Suggested Fix**: If you have ideas for how to fix it
- **Timeline**: When you discovered the issue
- **Disclosure Preferences**: Your preferences for credit/acknowledgment

#### Response Timeline

- **Initial Response**: Within 48-72 hours (Monday–Friday, 09:00–17:00 EST/EDT)
- **Status Updates**: Weekly until resolution (Monday–Friday, 09:00–17:00 EST/EDT)
- **Fix Timeline**: Depends on severity and complexity
- **Coordinated Disclosure**: Disclosure will be coordinated with the reporter and only after an agreed embargo or once a fix is available and tested

## Security Considerations

### What This Tool Does (and Doesn't Do)

#### Safe Operations

- **Generates test data only**: All output is fake, non-functional configuration data
- **No network access**: Tool operates entirely offline
- **No data collection**: No telemetry, logging, or data transmission
- **No in-place mutations**: Reads input files and writes outputs to new files; never overwrites existing configurations
- **Deterministic output**: Same inputs produce same outputs (when seeded)

#### Security Considerations

- **File system access**: Reads input files and writes new output files (no in-place edits or overwrites)
- **XML parsing**: Processes XML files which could contain malicious content
- **Memory usage**: Large configurations may consume significant memory
- **Temporary files**: May create temporary files during processing

#### What This Tool Cannot Do

- **Cannot access networks**: No internet connectivity or network scanning
- **Cannot execute code**: No code execution capabilities
- **Cannot access system resources**: No access to system files outside specified paths
- **Cannot persist data**: No database or persistent storage

## Security Best Practices

### For Users

1. **Validate Input Files**: Only use trusted XML files as base configurations
2. **Review Output**: Inspect generated configurations before using in test environments
3. **Use Sandboxed Environments**: Run the tool in isolated test environments
4. **Monitor Resource Usage**: Large configurations may require significant memory
5. **Keep Updated**: Use the latest stable release for security fixes

### For Developers

1. **Dependency Management**: Regularly update dependencies for security patches
2. **Input Validation**: All user inputs are validated and sanitized
3. **Error Handling**: Comprehensive error handling prevents information disclosure
4. **Memory Safety**: Rust's memory safety prevents common vulnerabilities
5. **No Unsafe Code**: The codebase forbids `unsafe` in CI (e.g., `#![forbid(unsafe_code)]` and lint checks)
6. **Fuzzing & Property Tests**: Fuzz parsers and generators (e.g., cargo-fuzz) and add property-based tests (e.g., proptest) for robustness

## Security Architecture

### Rust Security Features

This project leverages Rust's security features:

- **Memory Safety**: No buffer overflows, use-after-free, or data races
- **Type Safety**: Compile-time guarantees prevent many runtime errors
- **Zero-Cost Abstractions**: Security features without performance overhead
- **Safe Concurrency**: Thread-safe operations where applicable

### Dependency Security

#### Security Scanning

- **cargo-audit**: Regular vulnerability scanning of dependencies
- **GitHub Dependabot**: Automated security updates for dependencies
- **Manual Review**: Regular review of new dependencies

#### Dependency Policy

- **Minimal Dependencies**: Only essential dependencies are included
- **Well-Maintained**: Prefer actively maintained, widely-used crates
- **Security Focused**: Choose crates with good security practices
- **Regular Updates**: Keep dependencies updated to latest stable versions

## Security Contacts

### Primary Contact

- **GitHub Security Advisory**: Create a private security advisory
- **Response Time**: 48-72 hours (weekdays)
- **Process**: Use GitHub's built-in security advisory workflow

### Alternative Contact

- **Email**: `unclespider@pm.me` (if GitHub is unavailable)
- **Response Time**: 72-96 hours (weekdays)

## Security Resources

### For Users

- [Rust Security Best Practices](https://doc.rust-lang.org/book/ch00-00-introduction.html)
- [OPNsense Security Documentation](https://docs.opnsense.org/)
- [Network Security Fundamentals](https://www.rfc-editor.org/rfc/rfc4949)

### For Developers

- [Rust Security Guidelines](https://rust-lang.github.io/rust-clippy/master/)
- [Cargo Security Features](https://doc.rust-lang.org/cargo/)
- [Secure Coding Practices](https://owasp.org/www-project-secure-coding-practices-quick-reference-guide/)

### For Security Researchers

- [Responsible Disclosure Guidelines](https://www.first.org/global/sigs/vulnerability-coordination/responsible-disclosure)
- [Bug Bounty Best Practices](https://hackerone.com/bug-bounty-program)
- [Security Research Ethics](https://www.ieee.org/about/ieee-code-of-ethics.html)
