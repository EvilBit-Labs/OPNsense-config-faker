# GitHub Milestones and Issues for OPNsense Config Faker Rust Migration

This document provides the complete GitHub milestone and issue structure for organizing the Rust migration work, designed for consumption by AI agents.

---

## Milestone Definitions

### Milestone: v0.1 - Core CSV/CLI Foundation

**Description:** Establish foundation with clap CLI, VLAN/IP generator, CSV output, and strict CI/QA pipeline. This milestone focuses on creating the basic building blocks for the Rust implementation with comprehensive testing and quality enforcement.

**Due Date:** 2 weeks from start
**Labels:** `milestone-v0.1`, `foundation`, `cli`, `csv`

**Quality Gates:**

**Description:** Introduce XML output capabilities using quick-xml, implement basic configuration segments, and establish snapshot testing. This milestone creates the core XML generation functionality required for OPNsense configuration files.

**Due Date:** 4 weeks from start

**Labels:** `milestone-v0.2`, `xml`, `generation`, `templates`

**Quality Gates:**

- [ ] Generated XML validates against sample OPNsense configurations
- [ ] Unit test coverage ≥85%

---

**Description:** Implement cross-component validation and uniqueness constraints to ensure configuration integrity. Focus on preventing conflicts and ensuring generated configurations are internally consistent.

- [ ] All cross-component dependencies validated correctly
- [ ] Duplicate detection prevents invalid configurations
- [ ] Unit test coverage ≥90%

### Milestone: v0.4 - Concurrency and Performance

**Due Date:** 8 weeks from start

- [ ] TR001: Generate 1000+ VLAN configurations in <2 seconds

- [ ] Parallel processing shows measurable improvements

**Description:** Choose and implement either XSD validation via libxml2 or pure Rust typed models for schema compliance. This milestone addresses standards compliance and validation approach.
**Due Date:** 11 weeks from start
**Labels:** `milestone-v0.5`, `schema`, `validation`, `xsd`

**Quality Gates:**

- [ ] Generated configurations pass OPNsense import validation
- [ ] Performance impact of validation is minimal

### Milestone: v0.6 - Extended Networking Features

**Description:** Implement advanced OPNsense features including NAT rules, firewall configurations, CARP high availability, and RADIUS authentication. Focus on feature completeness and integration.

**Due Date:** 14 weeks from start

- [ ] NAT rules generated correctly for common use cases
- [ ] Firewall rules integrate with existing VLAN configurations

---

### Milestone: v0.7 - VPN Features Implementation

**Description:** Implement VPN technologies including WireGuard, OpenVPN, and IPSec with automated key generation, certificate management, and address allocation.
**Due Date:** 17 weeks from start
**Quality Gates:**

- [ ] Generated VPN configurations establish working tunnels
- [ ] Key generation follows security best practices
- [ ] VPN configurations integrate with firewall rules

---

**Description:** Final milestone delivering production-ready system with complete documentation, polished UX, cross-platform verification, and release automation.
**Labels:** `milestone-v1.0`, `production`, `documentation`, `release`
**Quality Gates:**

- [ ] Documentation covers all features with practical examples

- [ ] Binary releases work on all supported platforms
- [ ] Performance meets or exceeds all benchmarks

---

## Issue Templates for v0.1 Foundation

**Description:**

- [ ] CLI accepts basic commands: `generate csv`, `generate xml`
- [ ] Error messages are clear and actionable
- [ ] Command-line arguments are validated
- [ ] Supports `--count`, `--output-dir`, `--output-file` flags

**Technical Specifications:**

```rust
// Expected CLI structure
#[derive(Parser)]

}


    Generate(GenerateArgs),

}

**Dependencies:** None

**Definition of Done:**

- [ ] Unit tests cover all CLI parsing scenarios
- [ ] Documentation includes usage examples
- [ ] `cargo clippy -- -D warnings` passes

---

**Description:**
Implement VLAN ID generation and IP range allocation with uniqueness constraints and RFC compliance.

**Acceptance Criteria:**
- [ ] Allocate RFC 1918 compliant IP ranges
- [ ] Prevent duplicate VLAN IDs across generations
- [ ] Support department-based naming conventions

**Technical Specifications:**

```rust
pub struct VlanConfig {
    pub ip_range: IpNet,   // RFC 1918 networks
    pub description: String, // Department-based

    pub wan_interface: u8,  // WAN assignment
pub trait VlanGenerator {

}


**Dependencies:** None

**Definition of Done:**
- [ ] Integration tests with realistic data
- [ ] Performance tests for large generations

---

### Issue: [v0.1] CSV Writer - Data Serialization


**Description:**


- [ ] Serialize VlanConfig to CSV format
- [ ] Support custom output paths
- [ ] Support both file output and stdout

**Technical Specifications:**

```rust
pub struct CsvWriter {


}

impl CsvWriter {
}
```

**Dependencies:** VLAN Generator

**Definition of Done:**

- [ ] Integration tests with various output scenarios

**Labels:** `milestone-v0.1`, `testing`, `high-priority`, `infrastructure`

**Description:**
Establish comprehensive testing infrastructure with unit, integration, and property-based tests.

- [ ] Unit test framework setup with rstest

- [ ] Test coverage reporting
- [ ] Snapshot tests for output formats

```rust

// Example property-based test
#[rstest]
fn test_vlan_generation_uniqueness(#[case] count: usize) {
    let mut generator = VlanGenerator::new();
    let configs = generator.generate(count).unwrap();


    let vlan_ids: HashSet<_> = configs.iter().map(|c| c.id).collect();
```

**Definition of Done:**

- [ ] Documentation includes testing guidelines

---

### Issue: [v0.1] CI/CD Pipeline - Quality Enforcement

**Labels:** `milestone-v0.1`, `ci-cd`, `high-priority`, `infrastructure`

**Description:**
Setup GitHub Actions workflow with comprehensive quality gates including clippy, formatting, testing, and cross-platform builds.

- [ ] `cargo clippy -- -D warnings` enforcement
- [ ] `cargo fmt --check` validation
- [ ] Cross-platform builds (Linux, macOS, Windows)
- [ ] Test execution with coverage reporting
- [ ] Performance benchmark execution
**Technical Specifications:**

```yaml
# .github/workflows/ci.yml structure

name: CI

on: [push, pull_request]

jobs:
  quality:

      - uses: actions/checkout@v4
      - name: Setup Rust
      - name: Check formatting

    strategy:
        os: [ubuntu-latest, macos-latest, windows-latest]
    runs-on: ${{ matrix.os }}

  benchmark:
    runs-on: ubuntu-latest

    # Performance regression detection

**Dependencies:** All v0.1 components

- [ ] All quality gates pass

- [ ] Cross-platform builds successful

- [ ] Performance benchmarks tracked
- [ ] Badge integration in README
---

## Issue Templates for v0.2 XML Generation

### Issue: [v0.2] XML Writer - Core Generation Engine

**Labels:** `milestone-v0.2`, `xml`, `high-priority`, `generation`

**Description:**
Implement XML document generation using quick-xml with proper namespacing, formatting, and OPNsense configuration structure.


- [ ] Generate valid XML documents
- [ ] Support OPNsense configuration namespaces
- [ ] Implement proper XML formatting and indentation

- [ ] Support template-based generation

**Technical Specifications:**


    writer: Writer<W>,
    config: XmlWriterConfig,


impl<W: Write> XmlWriter<W> {
    pub fn write_opnsense_config(&mut self, config: &OpnsenseConfig) -> Result<(), XmlError>;
    pub fn write_vlan_section(&mut self, vlans: &[VlanConfig]) -> Result<(), XmlError>;
}
**Dependencies:** VLAN Generator

**Definition of Done:**

- [ ] Generated XML validates against OPNsense samples
- [ ] Unit tests cover all XML generation scenarios
- [ ] Integration tests with complex configurations
- [ ] Performance tests for large documents

---
### Issue: [v0.2] Configuration Templates - Base Structure


**Description:**
Create template system for different OPNsense configuration types with support for base configurations and custom sections.

**Acceptance Criteria:**

- [ ] Implement section-based template composition
- [ ] Allow custom template registration
- [ ] Support template inheritance and overrides
**Technical Specifications:**

```rust
pub struct ConfigTemplate {
    pub base: BaseConfig,

}

pub trait TemplateEngine {
    fn load_template(&mut self, name: &str) -> Result<ConfigTemplate, TemplateError>;
    fn render(&self, template: &ConfigTemplate, data: &ConfigData) -> Result<String, RenderError>;

}
```

**Dependencies:** XML Writer
**Definition of Done:**

- [ ] Template validation tests
- [ ] Rendering accuracy tests
- [ ] Performance tests for complex templates
- [ ] Documentation with template examples

## Additional Milestones Structure

*[Continue with similar detailed issue breakdowns for v0.3 through v1.0...]*

## AI Agent Guidelines

### Issue Assignment Strategy

2. **Dependency Management**: Ensure prerequisites are completed before dependent tasks
3. **Parallel Work**: Multiple agents can work on independent components simultaneously
4. **Integration Points**: Regular merge points to prevent divergence

- **Code Reviews**: Cross-agent review process for quality assurance
- **Integration Testing**: Collaborative testing at milestone boundaries

### Quality Assurance Process

- **Automated Validation**: CI pipeline enforces quality gates
- **Performance Monitoring**: Benchmark requirements tracked per milestone
- **Documentation**: Technical specifications and examples in each issue
