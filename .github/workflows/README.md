# CI Workflow Configuration

This directory contains GitHub Actions workflows for continuous integration.

## Current Workflows

### `ci.yml` - Main CI Pipeline

Runs on pushes to `main` and `develop` branches, and on pull requests to these branches.

**What it does:**

- Sets up Python 3.13 environment with UV package manager
- Installs development dependencies
- Runs linting, formatting, and type checking
- Executes tests with coverage reporting
- Uploads coverage reports to Codecov

## Required Secrets

### `CODECOV_TOKEN`

Required for uploading coverage reports to Codecov on protected branches.

**Setup:**

1. Go to [Codecov](https://codecov.io) and connect your repository
2. Copy the upload token from your repository settings
3. Add it as a secret named `CODECOV_TOKEN` in your GitHub repository settings:
   - Go to Settings → Secrets and variables → Actions
   - Click "New repository secret"
   - Name: `CODECOV_TOKEN`
   - Value: Your Codecov upload token

## Local Development

To run the same checks locally:

```bash
# Install dependencies
just install

# Run all CI checks
just ci-check

# Run tests with coverage
just test-cov
```

## Troubleshooting

### Pre-commit Installation Issues

- The workflow now explicitly installs pre-commit via pip
- If you encounter issues, ensure `pre-commit` is in your dev dependencies

### Coverage Upload Issues

- Ensure `CODECOV_TOKEN` secret is set
- Coverage reports are generated as `coverage.xml` in the root directory
- The workflow uses `fail_ci_if_error: false` to prevent CI failures from coverage upload issues

### Test Failures

- Tests are run with `--maxfail=1` to fail fast
- Coverage is still generated even if some tests fail
- Check the test output for specific failure details
