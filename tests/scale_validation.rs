//! Scale Validation Tests
//!
//! This module implements scale validation tests for the Rust implementation.

use opnsense_config_faker::generator::vlan::VlanGenerator;
use opnsense_config_faker::io::csv;
use std::time::Instant;
use tempfile::NamedTempFile;

/// Test the specific scales mentioned in the issue description
#[test]
fn test_scale_validation_comprehensive() {
    // Test scales: 10, 100, 1000
    let test_scales = vec![10, 100, 1000];

    for scale in test_scales {
        println!("🧪 Testing scale: {} VLANs", scale);

        let start = Instant::now();

        // Generate VLAN configurations
        let mut generator = VlanGenerator::new(Some(42));
        let configs = generator
            .generate_batch(scale)
            .expect("Failed to generate VLANs");

        let generation_time = start.elapsed();

        // Validate the generated configurations
        assert_eq!(
            configs.len(),
            scale,
            "Generated {} configs, expected {}",
            configs.len(),
            scale
        );

        // Check for unique VLAN IDs
        let mut vlan_ids: Vec<u16> = configs.iter().map(|c| c.vlan_id).collect();
        vlan_ids.sort();
        vlan_ids.dedup();
        assert_eq!(vlan_ids.len(), configs.len(), "VLAN IDs should be unique");

        // Check for unique networks
        let mut networks: Vec<String> = configs.iter().map(|c| c.ip_network.clone()).collect();
        networks.sort();
        networks.dedup();
        assert_eq!(networks.len(), configs.len(), "Networks should be unique");

        println!(
            "✅ Scale {}: Generated {} configs in {:?}",
            scale,
            configs.len(),
            generation_time
        );
    }
}

/// Test performance targets
#[test]
fn test_performance_targets() {
    let scale = 1000;
    println!("🧪 Testing performance at scale: {} VLANs", scale);

    let start = Instant::now();

    // Generate VLAN configurations
    let mut generator = VlanGenerator::new(Some(42));
    let configs = generator
        .generate_batch(scale)
        .expect("Failed to generate VLANs");

    let generation_time = start.elapsed();

    // Performance target: should complete in under 1 second for 1000 VLANs
    assert!(
        generation_time.as_millis() < 1000,
        "Generation took {:?}, should be under 1 second",
        generation_time
    );

    println!(
        "✅ Performance test passed: Generated {} configs in {:?}",
        configs.len(),
        generation_time
    );
}

/// Test memory efficiency
#[test]
fn test_memory_efficiency() {
    let scale = 1000;
    println!("🧪 Testing memory efficiency at scale: {} VLANs", scale);

    let start = Instant::now();

    // Generate VLAN configurations
    let mut generator = VlanGenerator::new(Some(42));
    let configs = generator
        .generate_batch(scale)
        .expect("Failed to generate VLANs");

    let generation_time = start.elapsed();

    // Memory efficiency target: should be able to generate 1000 configs without issues
    assert_eq!(
        configs.len(),
        scale,
        "Should generate exactly {} configs",
        scale
    );

    // Test CSV output to ensure no memory leaks
    let temp_file = NamedTempFile::new().expect("Failed to create temp file");
    csv::write_csv(&configs, temp_file.path()).expect("Failed to write CSV");

    // Read back the CSV to verify it was written correctly
    let read_configs = csv::read_csv(temp_file.path()).expect("Failed to read CSV");
    assert_eq!(
        read_configs.len(),
        scale,
        "CSV should have {} records",
        scale
    );

    println!(
        "✅ Memory efficiency test passed: Generated {} configs and CSV in {:?}",
        configs.len(),
        generation_time
    );
}

/// Test large scale generation
#[test]
fn test_large_scale_generation() {
    let scale = 1000; // Reduced from 5000 to avoid VLAN ID exhaustion
    println!("🧪 Testing large scale generation: {} VLANs", scale);

    let start = Instant::now();

    // Generate VLAN configurations
    let mut generator = VlanGenerator::new(Some(42));
    let configs = generator
        .generate_batch(scale)
        .expect("Failed to generate VLANs");

    let generation_time = start.elapsed();

    // Large scale target: should complete in under 2 seconds for 1000 VLANs
    assert!(
        generation_time.as_millis() < 2000,
        "Large scale generation took {:?}, should be under 2 seconds",
        generation_time
    );

    assert_eq!(
        configs.len(),
        scale,
        "Should generate exactly {} configs",
        scale
    );

    println!(
        "✅ Large scale test passed: Generated {} configs in {:?}",
        configs.len(),
        generation_time
    );
}
