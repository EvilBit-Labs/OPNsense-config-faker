//! Property-based tests for VLAN configuration generation
//!
//! This module contains property-based tests using the proptest framework to validate
//! core invariants of the VLAN configuration generation system. These tests ensure
//! that regardless of the input count or seed values, the generated configurations
//! always meet the specified requirements for uniqueness, validity, and determinism.

use opnsense_config_faker::generator::vlan::generate_vlan_configurations;
use opnsense_config_faker::generator::VlanConfig;
use proptest::prelude::*;
use std::collections::HashSet;

/// Generate a test strategy for valid VLAN counts in the range [1, 1000]
fn vlan_count_strategy() -> impl Strategy<Value = u16> {
    1u16..=1000u16
}

/// Generate a test strategy for seeds
fn seed_strategy() -> impl Strategy<Value = u64> {
    any::<u64>()
}

/// Validate that all VLAN IDs are unique and within the valid range [10, 4094]
fn validate_vlan_ids_unique_and_valid(configs: &[VlanConfig]) -> Result<(), String> {
    let mut seen_ids = HashSet::new();

    for config in configs {
        // Check valid range
        if !(10..=4094).contains(&config.vlan_id) {
            return Err(format!(
                "VLAN ID {} is outside valid range 10-4094",
                config.vlan_id
            ));
        }

        // Check uniqueness
        if !seen_ids.insert(config.vlan_id) {
            return Err(format!("Duplicate VLAN ID found: {}", config.vlan_id));
        }
    }

    Ok(())
}

/// Validate that all IP networks are unique and have the expected form
fn validate_ip_networks_unique_and_valid(configs: &[VlanConfig]) -> Result<(), String> {
    let mut seen_networks = HashSet::new();

    for config in configs {
        // Check expected format (either 10.x.y.x or 10.x.y.0/24)
        let is_x_format = config.ip_network.starts_with("10.")
            && config.ip_network.ends_with(".x")
            && config.ip_network.matches('.').count() == 3;

        let is_cidr_format = config.ip_network.starts_with("10.")
            && config.ip_network.ends_with(".0/24")
            && config.ip_network.matches('.').count() == 3;

        if !is_x_format && !is_cidr_format {
            return Err(format!(
                "IP network '{}' does not match expected format (10.x.y.x or 10.x.y.0/24)",
                config.ip_network
            ));
        }

        // Check uniqueness
        if !seen_networks.insert(&config.ip_network) {
            return Err(format!("Duplicate IP network found: {}", config.ip_network));
        }
    }

    Ok(())
}

/// Validate that all WAN assignments are in the valid range [1, 3]
fn validate_wan_assignments(configs: &[VlanConfig]) -> Result<(), String> {
    for config in configs {
        if !(1..=3).contains(&config.wan_assignment) {
            return Err(format!(
                "WAN assignment {} is outside valid range 1-3",
                config.wan_assignment
            ));
        }
    }

    Ok(())
}

/// Validate that gateway IP and DHCP range derivations succeed and are within the /24 network
fn validate_ip_derivations(configs: &[VlanConfig]) -> Result<(), String> {
    for config in configs {
        // Test gateway IP derivation
        let gateway_ip = config.gateway_ip().map_err(|e| {
            format!(
                "Failed to derive gateway IP for network '{}': {}",
                config.ip_network, e
            )
        })?;

        // Test DHCP range derivations
        let dhcp_start = config.dhcp_range_start().map_err(|e| {
            format!(
                "Failed to derive DHCP start for network '{}': {}",
                config.ip_network, e
            )
        })?;

        let dhcp_end = config.dhcp_range_end().map_err(|e| {
            format!(
                "Failed to derive DHCP end for network '{}': {}",
                config.ip_network, e
            )
        })?;

        // Validate that derived IPs are within the /24 network
        let network_prefix = if config.ip_network.ends_with(".x") {
            config.ip_network.strip_suffix(".x").unwrap()
        } else if config.ip_network.ends_with(".0/24") {
            config.ip_network.strip_suffix(".0/24").unwrap()
        } else {
            return Err(format!("Unexpected network format: {}", config.ip_network));
        };

        // Verify gateway IP format
        if !gateway_ip.starts_with(network_prefix) {
            return Err(format!(
                "Gateway IP '{gateway_ip}' is not within network prefix '{network_prefix}'"
            ));
        }

        // Verify DHCP range format
        if !dhcp_start.starts_with(network_prefix) {
            return Err(format!(
                "DHCP start IP '{dhcp_start}' is not within network prefix '{network_prefix}'"
            ));
        }

        if !dhcp_end.starts_with(network_prefix) {
            return Err(format!(
                "DHCP end IP '{dhcp_end}' is not within network prefix '{network_prefix}'"
            ));
        }

        // Verify the derived IPs have valid last octets
        let gateway_parts: Vec<&str> = gateway_ip.split('.').collect();
        let dhcp_start_parts: Vec<&str> = dhcp_start.split('.').collect();
        let dhcp_end_parts: Vec<&str> = dhcp_end.split('.').collect();

        if gateway_parts.len() != 4 || dhcp_start_parts.len() != 4 || dhcp_end_parts.len() != 4 {
            return Err("Derived IP addresses do not have 4 octets".to_string());
        }

        // Parse last octets and verify they're valid for a /24 network (0-255)
        let gateway_last: u8 = gateway_parts[3]
            .parse()
            .map_err(|_| format!("Invalid gateway IP last octet: {}", gateway_parts[3]))?;

        let dhcp_start_last: u8 = dhcp_start_parts[3]
            .parse()
            .map_err(|_| format!("Invalid DHCP start IP last octet: {}", dhcp_start_parts[3]))?;

        let dhcp_end_last: u8 = dhcp_end_parts[3]
            .parse()
            .map_err(|_| format!("Invalid DHCP end IP last octet: {}", dhcp_end_parts[3]))?;

        // Verify all octets are within valid range
        if gateway_last == 0 || gateway_last == 255 {
            return Err(format!(
                "Gateway IP last octet {gateway_last} is not valid for host addressing"
            ));
        }

        if dhcp_start_last == 0 || dhcp_start_last == 255 {
            return Err(format!(
                "DHCP start IP last octet {dhcp_start_last} is not valid for host addressing"
            ));
        }

        if dhcp_end_last == 0 || dhcp_end_last == 255 {
            return Err(format!(
                "DHCP end IP last octet {dhcp_end_last} is not valid for host addressing"
            ));
        }
    }

    Ok(())
}

proptest! {
    /// Test core invariants for any valid count and seed
    ///
    /// This property test validates that for any count in [1, 1000] and any seed,
    /// the generate_vlan_configurations function produces results that satisfy:
    /// - All VLAN IDs are unique and within range [10, 4094]
    /// - All IP networks are unique and follow the expected format
    /// - All WAN assignments are within range [1, 3]
    /// - Gateway IP and DHCP range derivations succeed and are within the /24 network
    #[test]
    fn test_vlan_generation_invariants(
        count in vlan_count_strategy(),
        seed in seed_strategy()
    ) {
        let configs = generate_vlan_configurations(count, Some(seed), None)
            .expect("Configuration generation should not fail for valid inputs");

        // Verify we got the expected number of configurations
        prop_assert_eq!(configs.len(), count as usize);

        // Validate all invariants
        validate_vlan_ids_unique_and_valid(&configs).map_err(TestCaseError::fail)?;
        validate_ip_networks_unique_and_valid(&configs).map_err(TestCaseError::fail)?;
        validate_wan_assignments(&configs).map_err(TestCaseError::fail)?;
        validate_ip_derivations(&configs).map_err(TestCaseError::fail)?;
    }

    /// Test determinism: same seed + same count produces identical sequences
    ///
    /// This property test validates that the configuration generation is deterministic
    /// by ensuring that multiple runs with the same parameters produce identical results.
    #[test]
    fn test_vlan_generation_determinism(
        count in 1u16..=100u16, // Smaller range for determinism tests
        seed in seed_strategy()
    ) {
        // Generate configurations twice with the same parameters
        let configs1 = generate_vlan_configurations(count, Some(seed), None)
            .expect("First generation should succeed");

        let configs2 = generate_vlan_configurations(count, Some(seed), None)
            .expect("Second generation should succeed");

        // Both should have the same length
        prop_assert_eq!(configs1.len(), configs2.len());

        // Both should be identical
        prop_assert_eq!(configs1, configs2,
            "Same seed and count should produce identical configurations");
    }

    /// Test uniqueness properties with smaller counts for faster execution
    #[test]
    fn test_uniqueness_properties(
        count in 1u16..=50u16,
        seed in seed_strategy()
    ) {
        let configs = generate_vlan_configurations(count, Some(seed), None)
            .expect("Configuration generation should succeed");

        // Collect all VLAN IDs and verify uniqueness
        let vlan_ids: HashSet<u16> = configs.iter().map(|c| c.vlan_id).collect();
        prop_assert_eq!(vlan_ids.len(), configs.len(),
            "All VLAN IDs should be unique");

        // Collect all IP networks and verify uniqueness
        let ip_networks: HashSet<&String> = configs.iter().map(|c| &c.ip_network).collect();
        prop_assert_eq!(ip_networks.len(), configs.len(),
            "All IP networks should be unique");
    }
}

// Tests gated behind the slow-tests feature for very large cases
#[cfg(feature = "slow-tests")]
mod slow_tests {
    use super::*;

    proptest! {
        /// Test with larger counts that might be slow but are important for completeness
        ///
        /// This test is gated behind the "slow-tests" feature to avoid slowing down
        /// regular test runs while still providing coverage for edge cases with large counts.
        #[test]
        fn test_large_vlan_generation_invariants(
            count in 500u16..=1000u16,
            seed in seed_strategy()
        ) {
            let configs = generate_vlan_configurations(count, Some(seed), None)
                .expect("Configuration generation should not fail for large counts");

            // Verify we got the expected number of configurations
            prop_assert_eq!(configs.len(), count as usize);

            // Validate all invariants (this may take longer for large counts)
            validate_vlan_ids_unique_and_valid(&configs).map_err(TestCaseError::fail)?;
            validate_ip_networks_unique_and_valid(&configs).map_err(TestCaseError::fail)?;
            validate_wan_assignments(&configs).map_err(TestCaseError::fail)?;
            validate_ip_derivations(&configs).map_err(TestCaseError::fail)?;
        }

        /// Test determinism with larger counts
        #[test]
        fn test_large_vlan_generation_determinism(
            count in 500u16..=1000u16,
            seed in seed_strategy()
        ) {
            // Generate configurations twice with the same parameters
            let configs1 = generate_vlan_configurations(count, Some(seed), None)
                .expect("First generation should succeed");

            let configs2 = generate_vlan_configurations(count, Some(seed), None)
                .expect("Second generation should succeed");

            // Both should be identical
            prop_assert_eq!(configs1, configs2,
                "Same seed and count should produce identical configurations even for large counts");
        }
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    /// Test the validation functions with known good data
    #[test]
    fn test_validation_functions_with_valid_data() {
        let config =
            VlanConfig::new(100, "10.1.2.x".to_string(), "Test VLAN 100".to_string(), 2).unwrap();

        let configs = vec![config];

        assert!(validate_vlan_ids_unique_and_valid(&configs).is_ok());
        assert!(validate_ip_networks_unique_and_valid(&configs).is_ok());
        assert!(validate_wan_assignments(&configs).is_ok());
        assert!(validate_ip_derivations(&configs).is_ok());
    }

    /// Test the validation functions with known bad data
    #[test]
    fn test_validation_functions_with_invalid_data() {
        // Create a config with invalid VLAN ID by bypassing normal constructor
        let mut config =
            VlanConfig::new(100, "10.1.2.x".to_string(), "Test".to_string(), 2).unwrap();
        config.vlan_id = 5; // Invalid VLAN ID

        let configs = vec![config];

        assert!(validate_vlan_ids_unique_and_valid(&configs).is_err());
    }

    /// Test validation with duplicate VLAN IDs
    #[test]
    fn test_validation_detects_duplicate_vlan_ids() {
        let config1 =
            VlanConfig::new(100, "10.1.2.x".to_string(), "Test 1".to_string(), 1).unwrap();

        let config2 = VlanConfig::new(
            100, // Same VLAN ID
            "10.1.3.x".to_string(),
            "Test 2".to_string(),
            2,
        )
        .unwrap();

        let configs = vec![config1, config2];

        assert!(validate_vlan_ids_unique_and_valid(&configs).is_err());
    }

    /// Test validation with duplicate IP networks
    #[test]
    fn test_validation_detects_duplicate_networks() {
        let config1 =
            VlanConfig::new(100, "10.1.2.x".to_string(), "Test 1".to_string(), 1).unwrap();

        let config2 = VlanConfig::new(
            101,
            "10.1.2.x".to_string(), // Same network
            "Test 2".to_string(),
            2,
        )
        .unwrap();

        let configs = vec![config1, config2];

        assert!(validate_ip_networks_unique_and_valid(&configs).is_err());
    }

    /// Test that small deterministic generation works as expected
    #[test]
    fn test_deterministic_generation_small_example() {
        let seed = 42u64;
        let count = 5u16;

        let configs1 = generate_vlan_configurations(count, Some(seed), None).unwrap();
        let configs2 = generate_vlan_configurations(count, Some(seed), None).unwrap();

        assert_eq!(configs1, configs2);
        assert_eq!(configs1.len(), 5);

        // Verify all generated configs meet the basic requirements
        for config in &configs1 {
            assert!((10..=4094).contains(&config.vlan_id));
            assert!((1..=3).contains(&config.wan_assignment));
            assert!(config.ip_network.starts_with("10."));
            assert!(config.ip_network.ends_with(".x") || config.ip_network.ends_with(".0/24"));
        }
    }

    /// Test edge case with count = 1
    #[test]
    fn test_single_configuration_generation() {
        let configs = generate_vlan_configurations(1, Some(12345), None).unwrap();
        assert_eq!(configs.len(), 1);

        let config = &configs[0];
        assert!((10..=4094).contains(&config.vlan_id));
        assert!((1..=3).contains(&config.wan_assignment));
        assert!(config.ip_network.starts_with("10."));

        // Test IP derivations
        assert!(config.gateway_ip().is_ok());
        assert!(config.dhcp_range_start().is_ok());
        assert!(config.dhcp_range_end().is_ok());
    }

    /// Test boundary count values
    #[test]
    fn test_boundary_count_values() {
        // Test minimum count
        let configs_min = generate_vlan_configurations(1, Some(123), None).unwrap();
        assert_eq!(configs_min.len(), 1);

        // Test a reasonable upper bound for regular tests
        let configs_upper = generate_vlan_configurations(100, Some(456), None).unwrap();
        assert_eq!(configs_upper.len(), 100);

        // Verify uniqueness for the larger set
        let vlan_ids: HashSet<u16> = configs_upper.iter().map(|c| c.vlan_id).collect();
        assert_eq!(vlan_ids.len(), 100, "All VLAN IDs should be unique");

        let ip_networks: HashSet<&String> = configs_upper.iter().map(|c| &c.ip_network).collect();
        assert_eq!(ip_networks.len(), 100, "All IP networks should be unique");
    }
}
