# Transition Note Update Guide for GitHub Issues #14–#31

## Overview

This document provides guidance for updating GitHub issues #14–#31 to include the standard Transition Note that communicates the Python→Rust migration status.

## Transition Note Template

Add the following section to all specified issues:

```markdown
---

## Transition Note

- This project is migrating from Python to Rust.
- Rust is the primary implementation target for new work. Python paths are maintained temporarily for operator parity.
- Please see Rust migration tracker issues #32–#41 for progress and cross-links.
- Backward compatibility references to Python remain during the transition and will be removed post-cutover.
```

## Issues Requiring Updates

Based on the analysis in `github-issues-language-migration-analysis.md`, the following issues need the Transition Note added:

### Issues Requiring Significant Updates (Python Implementation Details)

- **#20**: [FEATURE] Migrate to xsdata-generated models
- **#21**: [INFRASTRUCTURE] Project Setup and Development Environment
- **#22**: [ARCHITECTURE] Pydantic Model Integration Framework
- **#23**: [INFRASTRUCTURE] CLI Interface Foundation
- **#24**: [INFRASTRUCTURE] Error Handling and Validation
- **#28**: [TESTING] Unit Test Implementation and Coverage
- **#29**: [INFRASTRUCTURE] CI/CD Pipeline Implementation

### Issues Requiring Moderate Updates (Implementation Details)

- **#26**: [FEATURE] Progress Tracking and User Feedback
- **#27**: [FEATURE] Output Format and Size Validation
- **#30**: [DOCUMENTATION] Comprehensive Documentation Implementation

### Issues Requiring Minor Updates (Command Examples)

- **#14**: [FEATURE] XML Configuration Engine and Template Support
- **#15**: [FEATURE] Batch Processing and Output Management
- **#16**: [FEATURE] Realistic Data Generation and RFC Compliance
- **#17**: [FEATURE] Advanced NAT Configuration with Port Validation
- **#18**: [FEATURE] Configuration Options and Customization Framework
- **#19**: [FEATURE] Template System and Customization Framework

### Issues That Are Language-Agnostic (Minor Updates Only)

- **#25**: [FEATURE] Inbound NAT and Port Forwarding Generation
- **#31**: [TASK] Repository PM bootstrap (completed - may skip if closed)

## Update Process

For each issue #14–#31:

1. **Open the issue** in GitHub
2. **Edit the issue description**
3. **Scroll to the bottom** of the existing content
4. **Add the Transition Note section** as shown in the template above
5. **Save the changes**

## Additional Considerations

### Priority Order

Update in this priority order:

1. **Infrastructure issues first** (#21, #23, #24, #28, #29) - these are foundational
2. **Architecture issues** (#20, #22) - critical for development direction
3. **Feature issues** (#14-#19, #25-#27, #30) - user-facing functionality
4. **Completed issues last** (#31) - only if still open

### Consistency Notes

- Use the exact Transition Note template provided above
- Place the note at the end of each issue description
- Ensure the horizontal rule (`---`) separates it from existing content
- All references to issues #32–#41 should remain as specified

### Cross-References

The Transition Note references issues #32–#41 which are the Rust migration tracker issues. Ensure these issues exist before adding the references, or update the note accordingly if the issue numbers are different.

## Verification

After updating all issues, verify:

- [ ] All issues #14–#31 have the Transition Note added
- [ ] The note is consistently formatted across all issues
- [ ] Cross-references to migration tracker issues are correct
- [ ] No formatting issues or broken markdown in the updated descriptions

## Related Files

- `github-issues-language-migration-analysis.md` - Detailed analysis of which issues need updates
- `DEPENDENCIES.md` - Issue dependency matrix showing relationships
- Migration tracker issues #32–#41 - Referenced in the Transition Note
