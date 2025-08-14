//! Performance validation tests for OPNsense config generation
//!
//! These tests validate performance characteristics without affecting benchmark timing.

use opnsense_config_faker::generator::performance::PerformantConfigGenerator;
use opnsense_config_faker::generator::vlan::generate_vlan_configurations;
use std::time::Duration;

#[test]
fn test_vlan_id_uniqueness() {
    // Test that generated VLAN IDs are unique
    let mut generator = PerformantConfigGenerator::new(Some(42));
    let configs = generator.generate_batch(1000).unwrap();

    let mut ids = std::collections::HashSet::new();
    for config in &configs {
        assert!(
            ids.insert(config.vlan_id),
            "Duplicate VLAN ID found: {}",
            config.vlan_id
        );
    }
    assert_eq!(ids.len(), 1000);
}

#[test]
fn test_legacy_vlan_id_uniqueness() {
    // Test that legacy generator also produces unique VLAN IDs
    let configs = generate_vlan_configurations(1000, Some(42), None).unwrap();

    let mut ids = std::collections::HashSet::new();
    for config in &configs {
        assert!(
            ids.insert(config.vlan_id),
            "Duplicate VLAN ID found: {}",
            config.vlan_id
        );
    }
    assert_eq!(ids.len(), 1000);
}

#[test]
fn test_performance_regression_100_vlans() {
    // Performance regression test for 100 VLANs
    let start = std::time::Instant::now();
    let configs = generate_vlan_configurations(100, Some(42), None).unwrap();
    let duration = start.elapsed();

    // Get environment-aware threshold
    let is_ci = std::env::var("CI").is_ok();
    let threshold = if is_ci {
        Duration::from_millis(2000) // More lenient for CI
    } else {
        Duration::from_millis(1000) // Standard threshold
    };

    assert!(
        duration < threshold,
        "Performance regression: 100 VLANs took {:?} (threshold: {:?})",
        duration,
        threshold
    );
    assert_eq!(configs.len(), 100);
}

#[test]
fn test_throughput_validation() {
    // Throughput validation test
    let start = std::time::Instant::now();
    let configs = generate_vlan_configurations(150, Some(42), None).unwrap();
    let duration = start.elapsed();

    let throughput = configs.len() as f64 / duration.as_secs_f64();

    // Get environment-aware threshold
    let is_ci = std::env::var("CI").is_ok();
    let threshold = if is_ci {
        25.0 // Lower threshold for CI environment
    } else {
        50.0 // Standard threshold
    };

    assert!(
        throughput >= threshold,
        "Throughput too low: {:.2} configs/sec (threshold: {:.1})",
        throughput,
        threshold
    );
    assert_eq!(configs.len(), 150);
}

#[test]
fn test_optimized_throughput_validation() {
    // Optimized throughput validation test
    let start = std::time::Instant::now();
    let mut generator = PerformantConfigGenerator::new(Some(42));
    let configs = generator.generate_batch(150).unwrap();
    let duration = start.elapsed();

    let throughput = configs.len() as f64 / duration.as_secs_f64();

    // Get environment-aware threshold
    let is_ci = std::env::var("CI").is_ok();
    let threshold = if is_ci {
        50.0 // Lower threshold for CI environment
    } else {
        100.0 // Standard threshold
    };

    assert!(
        throughput >= threshold,
        "Optimized throughput too low: {:.2} configs/sec (threshold: {:.1})",
        throughput,
        threshold
    );
    assert_eq!(configs.len(), 150);
}
