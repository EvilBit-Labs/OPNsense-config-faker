//! Shared utilities for benchmarks with CI-aware optimizations
//!
//! This module provides helpers to automatically configure Criterion for
//! different environments (CI vs local development) to balance performance
//! feedback with CI speed.

use criterion::Criterion;
use std::time::Duration;

/// Detect if running in CI environment
pub fn is_ci() -> bool {
    std::env::var("CI").is_ok()
        || std::env::var("GITHUB_ACTIONS").is_ok()
        || matches!(
            std::env::var("BENCH_CI").as_deref(),
            Ok("1" | "true" | "yes")
        )
}

/// CI-optimized Criterion configuration while preserving full local defaults.
///
/// In CI:
/// - Reduces sample size from 100 to 30
/// - Reduces warmup time to 500ms
/// - Reduces measurement time to 2s
/// - Disables plot generation
/// - Sets noise threshold to 5%
///
/// Locally:
/// - Preserves all default Criterion behavior
/// - Respects CLI arguments via `configure_from_args()`
pub fn criterion_for_env() -> Criterion {
    let base = Criterion::default().configure_from_args();

    if is_ci() {
        // Tuned to cut CI time substantially but keep stability
        base.sample_size(30) // fewer samples than default (100)
            .warm_up_time(Duration::from_millis(500)) // reduced from 3s default
            .measurement_time(Duration::from_secs(2)) // reduced from 5s default
            .noise_threshold(0.05) // 5% noise threshold
            .without_plots() // skip plot generation in CI
    } else {
        // Keep rich/local experience (plots configurable via CLI args)
        base
    }
}

/// Choose dataset lists by environment.
///
/// Use smaller datasets in CI for faster execution while preserving
/// trend detection with at least 2 data points.
///
/// # Examples
/// ```rust
/// let sizes = ci_or_local(&[10, 100], &[10, 100, 500, 1000, 5000]);
/// ```
pub fn ci_or_local<T: Clone>(ci: &[T], local: &[T]) -> Vec<T> {
    if is_ci() {
        ci.to_vec()
    } else {
        local.to_vec()
    }
}

/// Cap a generated dataset in CI; pass-through locally.
///
/// Useful for limiting expensive generated test cases in CI
/// while allowing full exploration locally.
///
/// # Examples
/// ```rust
/// let test_cases = generate_test_cases(); // Returns iterator
/// let test_cases = cap_ci(test_cases, 250); // Limit to 250 in CI
/// ```
#[allow(dead_code)]
pub fn cap_ci<T>(items: impl IntoIterator<Item = T>, cap: usize) -> Vec<T> {
    if is_ci() {
        items.into_iter().take(cap).collect()
    } else {
        items.into_iter().collect()
    }
}

/// Get CI-appropriate iteration counts for different benchmark categories
#[allow(dead_code)]
pub fn ci_counts() -> BenchCounts {
    if is_ci() {
        BenchCounts {
            small: vec![10, 50],
            medium: vec![100, 500],
            large: vec![500, 1000],
            extra_large: vec![1000],
        }
    } else {
        BenchCounts {
            small: vec![10, 50, 100],
            medium: vec![100, 500, 1000],
            large: vec![1000, 5000, 10000],
            extra_large: vec![10000, 50000],
        }
    }
}

/// Predefined count sets for different benchmark scales
#[allow(dead_code)]
pub struct BenchCounts {
    pub small: Vec<usize>,
    pub medium: Vec<usize>,
    pub large: Vec<usize>,
    pub extra_large: Vec<usize>,
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::{cap_ci, ci_or_local, is_ci};

    #[test]
    fn test_ci_detection() {
        // Test with CI env var
        std::env::set_var("CI", "true");
        assert!(is_ci());
        std::env::remove_var("CI");

        // Test with GITHUB_ACTIONS
        std::env::set_var("GITHUB_ACTIONS", "true");
        assert!(is_ci());
        std::env::remove_var("GITHUB_ACTIONS");

        // Test with BENCH_CI
        std::env::set_var("BENCH_CI", "1");
        assert!(is_ci());
        std::env::remove_var("BENCH_CI");
    }

    #[allow(unused_variables)]
    #[test]
    fn test_ci_or_local() {
        std::env::set_var("CI", "true");
        let result = ci_or_local(&[1, 2], &[1, 2, 3, 4]);
        assert_eq!(result, vec![1, 2]);
        std::env::remove_var("CI");

        let result = ci_or_local(&[1, 2], &[1, 2, 3, 4]);
        assert_eq!(result, vec![1, 2, 3, 4]);
    }

    #[allow(unused_variables)]
    #[test]
    fn test_cap_ci() {
        std::env::set_var("CI", "true");
        let result = cap_ci(vec![1, 2, 3, 4, 5], 3);
        assert_eq!(result, vec![1, 2, 3]);
        std::env::remove_var("CI");

        let result = cap_ci(vec![1, 2, 3, 4, 5], 3);
        assert_eq!(result, vec![1, 2, 3, 4, 5]);
    }
}
