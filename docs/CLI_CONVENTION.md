# CLI Example Convention for Issues During Transition

This document standardizes CLI examples used in GitHub issues during the transition from Python to Rust implementation.

## Standard CLI Usage Template

Use this reusable snippet in GitHub issues under a "CLI Usage" section:

```md
## CLI Usage

### Rust Implementation
- **Development (cargo)**: `cargo run --release -- <subcommand> [flags]`
- **Installed binary**: `<binary-name> <subcommand> [flags]` *(replace `<binary-name>` once finalized)*

### Legacy Python (during transition)
- **Legacy path**: `python generate_csv.py [args]` *(kept for parity until full Rust cutover)*
```

## Example Usage

### Basic Example

```md
## CLI Usage

### Rust Implementation
- **Development (cargo)**: `cargo run --release -- generate --count 100 --output ./out`
- **Installed binary**: `<binary-name> generate --count 100 --output ./out` *(replace `<binary-name>` once finalized)*

### Legacy Python (during transition)
- **Legacy path**: `python generate_csv.py --count 100 --output ./out` *(kept for parity until full Rust cutover)*
```

### Complex Example with Multiple Subcommands

```md
## CLI Usage

### Rust Implementation
- **Development (cargo)**:
  - `cargo run --release -- analyze --input data.csv --format json`
  - `cargo run --release -- export --database ./db.sqlite --output report.pdf`
- **Installed binary**:
  - `<binary-name> analyze --input data.csv --format json` *(replace `<binary-name>` once finalized)*
  - `<binary-name> export --database ./db.sqlite --output report.pdf` *(replace `<binary-name>` once finalized)*

### Legacy Python (during transition)
- **Legacy path**:
  - `python analyze.py --input data.csv --format json` *(kept for parity until full Rust cutover)*
  - `python export.py --database ./db.sqlite --output report.pdf` *(kept for parity until full Rust cutover)*
```

## Acceptance Criteria for Color Output

All CLI implementations must respect terminal environment variables for color output:

### Required Environment Variable Support

- **`NO_COLOR`**: When set (any non-empty value), disable all color output
- **`TERM=dumb`**: When terminal is identified as "dumb", disable color output automatically

### Implementation Guidelines

- Model behavior after Python's Rich library, which automatically respects these variables
- Use appropriate Rust crates that support these standards (e.g., `colored`, `termcolor`, `ansi_term`)
- Test color output in various environments:
  - Standard terminal with color support
  - Terminal with `NO_COLOR=1` set
  - Terminal with `TERM=dumb` set
  - CI/CD environments (which typically set `TERM=dumb`)

### Testing Checklist

- [ ] Color output works in standard terminal
- [ ] `NO_COLOR=1 <command>` produces no color output
- [ ] `TERM=dumb <command>` produces no color output
- [ ] Color output is disabled automatically in CI environments
- [ ] Help text and error messages remain readable without color

## Backward Compatibility Notes

- Include both Rust and Python examples until full migration is complete
- Mark Python examples as "Legacy" to indicate transition status
- Maintain functional parity between implementations during transition
- Document any feature differences between Rust and Python versions

## Binary Name Placeholder

- Use `<binary-name>` as placeholder in all documentation
- Include note: "*(replace `<binary-name>` once finalized)*"
- Update all documentation once final binary name is decided
- Consider creating a find-and-replace checklist for the final naming

## Implementation Status Tracking

When using this convention in issues, consider adding implementation status:

```md
## Implementation Status
- [ ] Rust implementation complete
- [ ] Python legacy support maintained
- [ ] Color output respects NO_COLOR
- [ ] Color output respects TERM=dumb
- [ ] Documentation updated
- [ ] Binary name finalized
```
