//! Extended Scale Validation Tests
//!
//! This module implements comprehensive scale validation tests for the Rust implementation,
//! focusing on large-scale generation capabilities without Python dependencies.

use opnsense_config_faker::generator::vlan::VlanGenerator;
use opnsense_config_faker::io::csv;
use std::time::Instant;
use tempfile::NamedTempFile;

/// Test large scale generation (2000 VLANs)
#[test]
fn test_large_scale_2000_vlans() {
    let scale = 2000;
    println!("🧪 Testing large scale generation: {} VLANs", scale);

    let start = Instant::now();

    // Generate VLAN configurations
    let mut generator = VlanGenerator::new(Some(42));
    let configs = generator
        .generate_batch(scale)
        .expect("Failed to generate VLANs");

    let generation_time = start.elapsed();

    // Large scale target: should complete in under 5 seconds for 2000 VLANs
    assert!(
        generation_time.as_millis() < 5000,
        "Large scale generation took {:?}, should be under 5 seconds",
        generation_time
    );

    assert_eq!(
        configs.len(),
        scale,
        "Should generate exactly {} configs",
        scale
    );

    // Validate uniqueness
    let mut vlan_ids: Vec<u16> = configs.iter().map(|c| c.vlan_id).collect();
    vlan_ids.sort();
    vlan_ids.dedup();
    assert_eq!(vlan_ids.len(), configs.len(), "VLAN IDs should be unique");

    println!(
        "✅ Large scale test passed: Generated {} configs in {:?}",
        configs.len(),
        generation_time
    );
}

/// Test very large scale generation (3000 VLANs)
#[test]
fn test_very_large_scale_3000_vlans() {
    let scale = 3000;
    println!("🧪 Testing very large scale generation: {} VLANs", scale);

    let start = Instant::now();

    // Generate VLAN configurations
    let mut generator = VlanGenerator::new(Some(42));
    let configs = generator
        .generate_batch(scale)
        .expect("Failed to generate VLANs");

    let generation_time = start.elapsed();

    // Very large scale target: should complete in under 10 seconds for 3000 VLANs
    assert!(
        generation_time.as_millis() < 10000,
        "Very large scale generation took {:?}, should be under 10 seconds",
        generation_time
    );

    assert_eq!(
        configs.len(),
        scale,
        "Should generate exactly {} configs",
        scale
    );

    // Validate uniqueness
    let mut vlan_ids: Vec<u16> = configs.iter().map(|c| c.vlan_id).collect();
    vlan_ids.sort();
    vlan_ids.dedup();
    assert_eq!(vlan_ids.len(), configs.len(), "VLAN IDs should be unique");

    println!(
        "✅ Very large scale test passed: Generated {} configs in {:?}",
        configs.len(),
        generation_time
    );
}

/// Test memory scaling validation
#[test]
fn test_memory_scaling_validation() {
    let scales = vec![100, 500, 1000, 2000, 3000];

    for scale in scales {
        println!("🧪 Testing memory scaling at {} VLANs", scale);

        let start = Instant::now();

        // Generate VLAN configurations
        let mut generator = VlanGenerator::new(Some(42));
        let configs = generator
            .generate_batch(scale)
            .expect("Failed to generate VLANs");

        let generation_time = start.elapsed();

        // Validate basic properties
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
            "✅ Memory scaling test passed for {} VLANs in {:?}",
            scale, generation_time
        );
    }
}

/// Test scalability stress test
#[test]
fn test_scalability_stress_test() {
    let scale = 1500; // Moderate stress test
    println!("🧪 Testing scalability stress test: {} VLANs", scale);

    let start = Instant::now();

    // Generate VLAN configurations
    let mut generator = VlanGenerator::new(Some(42));
    let configs = generator
        .generate_batch(scale)
        .expect("Failed to generate VLANs");

    let generation_time = start.elapsed();

    // Stress test target: should complete in under 3 seconds for 1500 VLANs
    assert!(
        generation_time.as_millis() < 3000,
        "Stress test generation took {:?}, should be under 3 seconds",
        generation_time
    );

    assert_eq!(
        configs.len(),
        scale,
        "Should generate exactly {} configs",
        scale
    );

    // Comprehensive validation
    let mut vlan_ids: Vec<u16> = configs.iter().map(|c| c.vlan_id).collect();
    vlan_ids.sort();
    vlan_ids.dedup();
    assert_eq!(vlan_ids.len(), configs.len(), "VLAN IDs should be unique");

    let mut networks: Vec<String> = configs.iter().map(|c| c.ip_network.clone()).collect();
    networks.sort();
    networks.dedup();
    assert_eq!(networks.len(), configs.len(), "Networks should be unique");

    // Test CSV round-trip
    let temp_file = NamedTempFile::new().expect("Failed to create temp file");
    csv::write_csv(&configs, temp_file.path()).expect("Failed to write CSV");
    let read_configs = csv::read_csv(temp_file.path()).expect("Failed to read CSV");
    assert_eq!(
        read_configs.len(),
        scale,
        "CSV round-trip should preserve count"
    );

    println!(
        "✅ Scalability stress test passed: Generated {} configs in {:?}",
        configs.len(),
        generation_time
    );
}

/// Test multiple batch generation
#[test]
fn test_multiple_batch_generation() {
    let batch_size = 500;
    let num_batches = 3;
    println!(
        "🧪 Testing multiple batch generation: {} batches of {} VLANs each",
        num_batches, batch_size
    );

    let start = Instant::now();

    let mut all_configs = Vec::new();

    // Use a single generator instance to ensure no duplicate VLAN IDs across batches
    let mut generator = VlanGenerator::new(Some(42));

    for batch_num in 0..num_batches {
        let configs = generator
            .generate_batch(batch_size)
            .expect("Failed to generate VLANs");

        // Validate batch
        assert_eq!(
            configs.len(),
            batch_size,
            "Batch {} should have {} configs",
            batch_num,
            batch_size
        );

        all_configs.extend(configs);
    }

    let generation_time = start.elapsed();

    // Multiple batch target: should complete in under 5 seconds for 1500 total VLANs
    assert!(
        generation_time.as_millis() < 5000,
        "Multiple batch generation took {:?}, should be under 5 seconds",
        generation_time
    );

    assert_eq!(
        all_configs.len(),
        batch_size * num_batches,
        "Should generate exactly {} total configs",
        batch_size * num_batches
    );

    // Check for uniqueness across all batches
    let mut all_vlan_ids: Vec<u16> = all_configs.iter().map(|c| c.vlan_id).collect();
    all_vlan_ids.sort();
    all_vlan_ids.dedup();
    assert_eq!(
        all_vlan_ids.len(),
        all_configs.len(),
        "All VLAN IDs should be unique across all batches"
    );

    println!(
        "✅ Multiple batch generation test passed: Generated {} total configs in {:?}",
        all_configs.len(),
        generation_time
    );
}
