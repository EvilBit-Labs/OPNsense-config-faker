# OPNsense Config Faker - Issue Dependency Matrix

This document outlines the interdependencies between GitHub issues to help with development planning and sequencing.

## 🏗️ Dependency Levels

### Level 1: Foundation (No Dependencies)

- **#21** - Project Setup and Development Environment
  - **Blocks**: #20, #23, #24, #28, #29

### Level 2: Core Architecture (Depends on Level 1)

- **#20** - Migrate to xsdata-generated models

  - **Depends on**: #21
  - **Blocks**: #22, #14, #1-#6, #10-#13

- **#23** - CLI Interface Foundation

  - **Depends on**: #21
  - **Blocks**: #18, #15, #26

- **#24** - Error Handling and Validation

  - **Depends on**: #21
  - **Blocks**: All feature issues (validation framework needed)

### Level 3: Integration Framework (Depends on Level 2)

- **#22** - Pydantic Model Integration Framework

  - **Depends on**: #20
  - **Blocks**: #14, #15, #1-#6, #13

- **#28** - Unit Test Implementation

  - **Depends on**: #21, #20, #22
  - **Blocks**: #29

### Level 4: Core Features (Depends on Level 3)

- **#14** - XML Configuration Engine and Template Support

  - **Depends on**: #20, #22
  - **Blocks**: #1-#6, #13

- **#16** - Realistic Data Generation and RFC Compliance

  - **Depends on**: #20, #22
  - **Blocks**: #1-#6

- **#18** - Configuration Options and Customization Framework

  - **Depends on**: #23
  - **Blocks**: All generation features

### Level 5: Feature Generation (Depends on Level 4)

- **#1** - Firewall Rules with Realistic Patterns
- **#2** - DHCP Server Configurations with Realistic Scopes
- **#3** - Interface Configurations with Realistic Naming
- **#4** - NAT Rules with Port Mappings
- **#5** - CARP Virtual IP Configurations
- **#6** - RADIUS User Accounts with Authentication Details
- **#13** - VPN Configuration Generation

All depend on: #20, #22, #14, #16

### Level 6: Validation and Consistency (Depends on Level 5)

- **#10** - Ensure Generated Configurations are Internally Consistent
- **#11** - Validate IP Address Assignments Don't Conflict
- **#12** - Check VLAN ID Uniqueness Across All Components

All depend on: Core feature generation (#1-#6)

### Level 7: Advanced Integration (Depends on Level 6)

- **#7** - Cross-Reference VLANs with Appropriate Interfaces
- **#8** - Link DHCP Scopes to Corresponding VLAN Networks
- **#9** - Generate Consistent Firewall Rules Based on Network Topology

All depend on: #2, #3, #12 and validation framework

### Level 8: Enhanced Features (Can be developed in parallel)

- **#15** - Batch Processing and Output Management
- **#17** - Advanced NAT Configuration with Port Validation
- **#25** - Inbound NAT and Port Forwarding Generation
- **#26** - Progress Tracking and User Feedback
- **#27** - Output Format and Size Validation
- **#30** - Comprehensive Documentation Implementation

### Level 9: Infrastructure (Depends on multiple levels)

- **#29** - CI/CD Pipeline Implementation
  - **Depends on**: #21, #28
  - **Timeline**: After testing framework is established

## 🚦 Critical Path Analysis

### Phase 1 (v1.0 Foundation)

1. **#21** → **#20** → **#22** → **#14**
2. **#21** → **#23** → **#18**
3. **#21** → **#24** (parallel with above)
4. **#28** → **#29** (after models established)

### Phase 2 (v1.0 Features)

After Phase 1 complete:

- **#16** → **#1, #2, #3, #4, #5, #6** (core features)
- **#15** (batch processing)

### Phase 3 (v1.1 Validation)

After Phase 2 complete:

- **#10, #11, #12** (validation features)
- **#13** (VPN generation)
- **#17, #25** (advanced NAT)

### Phase 4 (v1.2 Integration)

After Phase 3 complete:

- **#7, #8, #9** (topology integration)

## 📋 Recommended Development Sequence

01. **Start**: #21 (Project Setup)
02. **Architecture**: #20 (xsdata models)
03. **Integration**: #22 (Pydantic framework)
04. **Infrastructure**: #23, #24 (CLI, Error handling)
05. **Testing**: #28 (Unit tests)
06. **Automation**: #29 (CI/CD)
07. **Core Engine**: #14 (XML engine)
08. **Data Quality**: #16 (Realistic data)
09. **Configuration**: #18 (Options framework)
10. **Features**: #1-#6 (Core features in parallel)
11. **Validation**: #10-#12 (Consistency checks)
12. **Advanced**: #7-#9 (Topology integration)

## 🏷️ Suggested Dependency Labels

Consider adding these GitHub labels for dependency tracking:

- `dependency:foundation` - No dependencies, foundational work
- `dependency:architecture` - Depends on foundation
- `dependency:features` - Depends on architecture
- `dependency:integration` - Depends on features
- `blocked-by:issue-N` - Blocked by specific issue number
- `blocks:multiple` - Blocks multiple other issues
- `critical-path` - On the critical development path
