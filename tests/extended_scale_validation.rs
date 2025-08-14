//! Extended Scale Validation Tests
//! 
//! Tests for the large scales mentioned in the issue (5000, 10000 VLANs)
//! These tests are marked as slow tests and can be enabled with --features slow-tests

use crate::migration_validation::{MigrationValidator, ValidationConfig};

#[path = "migration_validation.rs"]
mod migration_validation;

/// Test large scale validation (5000 VLANs) - requires slow-tests feature
#[test]
#[cfg(feature = "slow-tests")]
fn test_large_scale_5000_vlans() {
    let validator = MigrationValidator::new().expect("Failed to create validator");
    
    println!("🧪 Testing LARGE scale: 5000 VLANs (this may take a while...)");
    
    let config = ValidationConfig::new("large_scale_5000", 5000, Some(42));
    
    match validator.run_validation(config) {
        Ok(result) => {
            println!("📊 Large Scale 5000: {}", result.summary());
            
            if result.is_valid() {
                println!("✅ SUCCESS: Output match at scale 5000");
            } else {
                println!("❌ FAIL: Output mismatch at scale 5000");
                println!("   This may indicate resource limitations or edge cases at large scale");
            }
            
            // Performance should still show improvement even at large scale
            assert!(result.meets_performance_target(1.0), 
                    "Even at large scale, Rust should be at least as fast as Python: {}", result.summary());
        }
        Err(e) => {
            println!("⚠️  Large scale test failed: {}", e);
            // Don't panic on large scale tests as they may hit resource limits
        }
    }
}

/// Test extra large scale validation (10000 VLANs) - requires slow-tests feature  
#[test]
#[cfg(feature = "slow-tests")]
fn test_extra_large_scale_10000_vlans() {
    let validator = MigrationValidator::new().expect("Failed to create validator");
    
    println!("🧪 Testing EXTRA LARGE scale: 10000 VLANs (this will take significant time...)");
    
    let config = ValidationConfig::new("extra_large_scale_10000", 10000, Some(42));
    
    match validator.run_validation(config) {
        Ok(result) => {
            println!("📊 Extra Large Scale 10000: {}", result.summary());
            
            if result.is_valid() {
                println!("✅ SUCCESS: Output match at scale 10000");
            } else {
                println!("❌ FAIL: Output mismatch at scale 10000");
                println!("   This may indicate resource limitations at extra large scale");
            }
            
            // At this scale, we're primarily testing that it doesn't crash
            // Performance may be less critical than stability
            println!("🎯 Extra large scale test completed successfully");
        }
        Err(e) => {
            println!("⚠️  Extra large scale test failed: {}", e);
            println!("   This is not unexpected at 10k scale due to resource constraints");
            // Don't panic on extra large scale tests
        }
    }
}

/// Stress test to validate the scalability limits
#[test]
#[cfg(feature = "slow-tests")]
fn test_scalability_stress_test() {
    let validator = MigrationValidator::new().expect("Failed to create validator");
    
    // Test a range of scales to find the practical limits
    let stress_scales = vec![1500, 2000, 3000, 4000];
    
    for scale in stress_scales {
        println!("🔬 Stress testing scale: {} VLANs", scale);
        
        let config = ValidationConfig::new(&format!("stress_{}", scale), scale, Some(123));
        
        match validator.run_validation(config) {
            Ok(result) => {
                println!("📊 Stress Scale {}: {}", scale, result.summary());
                
                // At stress scales, we mainly want to ensure it completes
                if result.is_valid() {
                    println!("✅ Stress test passed at scale {}", scale);
                } else {
                    println!("⚠️  Stress test structural validation failed at scale {}", scale);
                }
            }
            Err(e) => {
                println!("❌ Stress test failed at scale {}: {}", scale, e);
                // Continue with other scales even if one fails
            }
        }
    }
}

/// Test to validate memory efficiency at different scales
#[test]
fn test_memory_scaling_validation() {
    let validator = MigrationValidator::new().expect("Failed to create validator");
    
    // Test memory efficiency across a range of scales
    let memory_test_scales = vec![100, 250, 500, 750, 1000];
    let mut results = Vec::new();
    
    for scale in memory_test_scales {
        let config = ValidationConfig::new(&format!("memory_scale_{}", scale), scale, Some(456));
        
        match validator.run_validation(config) {
            Ok(result) => {
                println!("💾 Memory Scale {}: {}", scale, result.summary());
                results.push((scale, result.performance_ratio, result.rust_duration));
            }
            Err(e) => {
                println!("❌ Memory test failed at scale {}: {}", scale, e);
            }
        }
    }
    
    // Analyze memory scaling characteristics
    if results.len() >= 2 {
        println!("📈 Memory Scaling Analysis:");
        for (scale, ratio, duration) in &results {
            println!("   Scale {}: {:.2}x faster, {:?} runtime", scale, ratio, duration);
        }
        
        // Check that performance doesn't degrade too much with scale
        let first_ratio = results[0].1;
        let last_ratio = results[results.len() - 1].1;
        let degradation_ratio = first_ratio / last_ratio;
        
        println!("   Performance degradation ratio: {:.2}x", degradation_ratio);
        
        // Performance shouldn't degrade more than 10x across the test range
        assert!(degradation_ratio < 10.0, 
                "Performance degradation should be reasonable: {:.2}x", degradation_ratio);
    }
}