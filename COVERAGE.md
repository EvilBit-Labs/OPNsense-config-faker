# Coverage Tooling

This project uses cargo-llvm-cov for code coverage analysis with a >90% coverage threshold.

## Setup

Install the coverage tooling dependencies:

```bash
just setup
```

This will install:

- `rustfmt` and `clippy` components
- `cargo-llvm-cov` for coverage analysis

## Running Coverage

### Local Development

```bash
# Generate coverage report with 90% threshold
just coverage

# Generate HTML coverage report for local viewing
just coverage-html

# View coverage report in terminal
just coverage-report
```

The HTML report will be available at `target/llvm-cov/html/index.html` after running `just coverage-html`.

### CI/CD

The CI system automatically runs coverage analysis:

```bash
# CI-friendly coverage (ignores test failures but still generates coverage)
just coverage-ci
```

## Coverage Files

- `lcov.info` - Coverage data in LCOV format (uploaded to Codecov)
- `target/llvm-cov/html/` - HTML coverage reports
- Coverage artifacts are excluded from version control via `.gitignore`

## Coverage Configuration

### Included in Coverage

- Library code (`src/lib.rs` and modules)
- Integration tests (`tests/`)
- Unit tests (within modules)
- Doctests (documentation examples)

### Excluded from Coverage

- Benchmarks (`benches/`) - Automatically excluded as separate targets
- Generated files
- Test utilities (helper code in `tests/common/`)

## Thresholds

- **Line Coverage**: >90% required (enforced by `--fail-under-lines 90`)
- Coverage failures will cause CI builds to fail
- Use `just coverage-ci` for CI environments to generate reports even with test failures

## Troubleshooting

### Coverage Too Low

If coverage drops below 90%:

1. Add more unit tests for uncovered code
2. Add integration tests for user-facing functionality
3. Add doctests for public APIs
4. Remove dead/unreachable code

### View Missing Coverage

```bash
just coverage-html
# Open target/llvm-cov/html/index.html in browser
```

The HTML report shows exactly which lines are not covered.

### Clean Coverage Data

```bash
just coverage-clean
```

## Commands Reference

| Command                 | Description                                           |
| ----------------------- | ----------------------------------------------------- |
| `just coverage`         | Run tests with coverage and enforce 90% threshold     |
| `just coverage-ci`      | Run coverage for CI (ignores test failures)           |
| `just coverage-html`    | Generate HTML coverage report                         |
| `just coverage-html-ci` | Generate HTML coverage report (ignores test failures) |
| `just coverage-report`  | Show coverage report in terminal                      |
| `just coverage-clean`   | Clean coverage artifacts                              |
| `just test-doc`         | Run doctests only                                     |
| `just test-unit`        | Run unit tests only                                   |
| `just test-integration` | Run integration tests only                            |
