@AGENTS.md

## Session Learnings

### Dependency Management

- Cargo.toml may be reformatted by `cargo-sort` pre-commit hook — run `just ci-check` twice if it fails on "Cargo.toml is sorted"
- Never bump `rand` or `rand_chacha` without updating all generator code — rand 0.9→0.10 is a breaking API change
- Always verify the original code compiles (`git stash && cargo check`) before "fixing" reported build issues
- Cargo.lock is checked in — restore it with `git checkout HEAD -- Cargo.lock` if dep resolution changes unexpectedly

### CI Workflow

- `just ci-check` is the mandatory gate before commits — includes pre-commit hooks, clippy, tests, coverage, and dist-plan
- `dist-plan` may fail due to `cargo-dist` version mismatch — this is an environment issue, not a code issue
- Pre-commit hooks auto-fix files (cargo-sort, fmt) — a second run should pass after auto-fixes

### Code Patterns

- Error message strings are asserted in both unit tests (`src/`) and integration tests (`tests/`) — changing an error message requires updating both
- `VlanConfig` fields are `pub` — mutations bypass `new()` validation (known tech debt, tracked in issue #105)
- XML template path (`xml/template.rs`) is the production code path; `xml/{builder,engine,injection,generator}.rs` are unused
- `escape_xml_string` in `xml/template.rs` is the canonical XML escaping function (re-exported from `xml/mod.rs`) — do not duplicate
- VPN/NAT CSV export is **not implemented** — `generate.rs` generates data in memory but does not write files
- `vlan.rs` (1668 lines) exceeds 800-line limit — consolidation tracked in issue #113

### GitHub

- Label `performance` does not exist — use `enhancement` for perf-related issues
- Open refactoring issues: #109 (XML template), #110 (dept caching), #111 (benchmarks), #112 (streaming XML), #113 (vlan.rs size), #114 (CLI dedup), #115 (vpn.rs types)

### Git

- Default push may try all tracking branches — `git push origin <branch>` to push only the feature branch
- DCO sign-off required: `git commit -s`
