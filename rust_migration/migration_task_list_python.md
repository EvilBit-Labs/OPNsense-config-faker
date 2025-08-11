# Migration Task List

## Phase 1: Foundation Setup

### Step 1: Generate Models

- [ ] Use `xsdata` to generate Pydantic models from the `opnsense-config.xsd`.
- [ ] Store generated models in the `opnsense/models` directory.

### Step 2: Create Model Factories

- [ ] Develop factory classes for each module (`Interface`, `DHCP`, `NAT`, etc.) in the `opnsense/factories` directory.
- [ ] Integrate `Faker` for generating realistic test data within models.

### Step 3: Establish Testing Infrastructure

- [ ] Set up tests to validate model generation.
- [ ] Ensure models align with legacy structures for backward compatibility.

## Phase 2: Core Migration

### Step 4: Migrate Legacy Generators

- [ ] Refactor `generateParts()` in `generateXMLConfig.py` to use newly created models.
- [ ] Update individual generator scripts (`genInterface.py`, `genDHCP.py`, etc.) to utilize model factories.

### Step 5: Implement Structured Serialization

- [ ] Integrate schema validation to ensure configurations conform to `opnsense-config.xsd`.
- [ ] Replace `insertPartsToConfig()` function with model serialization logic.

### Step 6: Update CLI Integration

- [ ] Refactor `main.py` to call new model-based generation functions for XML creation.

## Phase 3: Testing & Validation

### Step 7: Conduct Comprehensive Testing

- [ ] Develop tests for new model-based generation processes.
- [ ] Compare outputs of legacy and new systems for functional equivalence.

### Step 8: Implement CI/CD Pipelines

- [ ] Integrate schema validation in the CI/CD pipeline.
- [ ] Perform performance testing to confirm no regressions exist.

### Step 9: Optimize Performance

- [ ] Optimize CSV processing.
- [ ] Enhance XML serialization for efficiency and performance.

## Phase 4: Cleanup & Optimization

### Step 10: Remove Legacy Code

- [ ] Delete old XML template files and outdated generator scripts.
- [ ] Clean up any remaining XML string manipulation logic.

### Step 11: Enhance Validation

- [ ] Implement additional checks for IP conflicts and VLAN uniqueness.
- [ ] Expand validation to cover all potential configuration inconsistencies.

---

**Note:** Each step should be thoroughly documented with any changes made to functions or module integrations to maintain a clear record of the migration process.
