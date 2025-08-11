# GitHub Actions CI/CD Configuration

This directory contains the CI/CD pipeline configuration for the OPNsense Config Faker project.

## Workflows

### CI Pipeline (`ci.yml`)

The main CI pipeline runs on all pushes and pull requests to `main` and `develop` branches. It includes:

#### Jobs

1. **Semantic PR Title Check** (`semantic-pr`)

   - Validates that PR titles follow [Conventional Commits](https://www.conventionalcommits.org/) specification
   - Supported types: `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `build`, `ci`, `chore`
   - Only runs on pull requests

2. **Lint** (`lint`)

   - Checks code formatting with `cargo fmt --all -- --check`
   - Runs Clippy linter with `cargo clippy --all-targets --all-features -- -D warnings`
   - Uses strict linting settings to enforce code quality

3. **Test and Coverage** (`test-and-coverage`)

   - Runs all tests with coverage collection using `cargo-llvm-cov`
   - Enforces minimum 90% code coverage with `--fail-under 90`
   - Generates both LCOV and HTML coverage reports
   - Uploads coverage artifacts for 30 days
   - Optionally uploads coverage to Codecov (requires `CODECOV_TOKEN` secret)

4. **Benchmarks** (`benches`)

   - Runs performance benchmarks using Criterion
   - Compares benchmark results against base branch for PRs
   - Tracks benchmark performance over time on main/develop branches
   - Uploads HTML benchmark reports as artifacts
   - Comments on PRs with benchmark results and artifact links

#### Environment Variables

- `CARGO_TERM_COLOR=never`: Disables colored output for consistent CI logs
- `TERM=dumb`: Ensures terminal compatibility in CI environment

#### Caching Strategy

Each job uses optimized Rust caching with `Swatinem/rust-cache@v2` to speed up builds:

- Separate cache keys for different job types (`lint-`, `test-`, `bench-`)
- Caches Cargo registry, Git dependencies, and build artifacts
- Automatically handles cache invalidation based on `Cargo.toml` changes

## Code Review Configuration

### CODEOWNERS

- All files are owned by `@UncleSp1d3r`
- Configured to prefer CodeRabbit over GitHub Copilot for code reviews
- Covers source code, tests, CI/CD files, and documentation

### Dependabot

- Automatically updates Rust dependencies weekly (Mondays at 06:00)
- Also updates GitHub Actions weekly
- Creates PRs with conventional commit messages
- Assigns PRs to `@UncleSp1d3r` for review
- Limits concurrent PRs to prevent notification spam

## Artifacts

The CI pipeline generates several useful artifacts:

1. **Coverage Reports** (30-day retention):

   - `lcov-report`: LCOV coverage data
   - `html-coverage-report`: Human-readable HTML coverage report

2. **Benchmark Reports** (30-day retention):

   - `criterion-html-reports-{sha}`: Detailed Criterion benchmark reports
   - Access via Actions tab → Select workflow run → Download artifacts

## Secrets Configuration

To enable all features, configure these repository secrets:

- `CODECOV_TOKEN`: Optional token for Codecov integration
- `GITHUB_TOKEN`: Automatically provided by GitHub Actions

## Usage Notes

- The pipeline runs automatically on pushes and PRs
- Coverage must be ≥90% or the build fails
- Clippy warnings are treated as errors (`-D warnings`)
- All formatting must pass `cargo fmt` checks
- PR titles must follow conventional commits format

## Performance Monitoring

- Benchmark results are tracked over time
- PRs show performance comparisons against the base branch
- Significant performance regressions (>150%) trigger alerts
- HTML reports provide detailed performance analysis

## Repository Settings Recommendations

For optimal CodeRabbit integration:

1. **Branch Protection Rules**:

   - Require status checks to pass before merging
   - Require up-to-date branches before merging
   - Include administrators in restrictions

2. **Code Review Settings**:

   - Disable GitHub Copilot auto-reviews if configured
   - Enable CodeRabbit integration via marketplace

3. **Notifications**:

   - Configure appropriate notification settings for CI failures
   - Set up Slack/email integration if needed

## Troubleshooting

### Common Issues

1. **Coverage Below 90%**: Add more tests or exclude non-testable code with `#[cfg(not(test))]`
2. **Clippy Failures**: Fix warnings or use `#[allow(clippy::lint_name)]` for justified cases
3. **Format Failures**: Run `cargo fmt` locally before pushing
4. **Benchmark Failures**: Check for significant performance regressions

### Local Testing

Before pushing, run these commands locally:

```bash
# Check formatting
cargo fmt --all -- --check

# Run linting
cargo clippy --all-targets --all-features -- -D warnings

# Run tests with coverage
cargo llvm-cov --all-features --workspace --lcov --fail-under 90

# Run benchmarks
cargo bench
```
