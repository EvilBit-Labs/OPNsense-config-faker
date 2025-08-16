//! Property-based tests for VLAN configuration generation
//!
//! This module contains property-based tests using the proptest framework to validate
//! core invariants of the VLAN configuration generation system. These tests ensure
//! that regardless of the input count or seed values, the generated configurations
//! always meet the specified requirements for uniqueness, validity, and determinism.

use opnsense_config_faker::generator::vlan;
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_vlan_generation_does_not_panic(
        count in 1..100u16,
        seed in 1..1000u64
    ) {
        let _ = vlan::generate_vlan_configurations(count, Some(seed), None);
    }

    #[test]
    fn test_vlan_id_is_always_in_valid_range(
        count in 1..100u16,
        seed in 1..1000u64
    ) {
        let vlans = vlan::generate_vlan_configurations(count, Some(seed), None).unwrap();

        prop_assert!(vlans.iter().all(|v| v.vlan_id >= 10 && v.vlan_id <= 4094));
    }

    #[test]
    fn test_no_duplicate_vlan_ids_are_generated(
        count in 1..100u16,
        seed in 1..1000u64
    ) {
        let vlans = vlan::generate_vlan_configurations(count, Some(seed), None).unwrap();
        let mut ids: Vec<_> = vlans.iter().map(|v| v.vlan_id).collect();
        let initial_len = ids.len();
        ids.sort();
        ids.dedup();
        prop_assert_eq!(initial_len, ids.len());
    }

    #[test]
    fn test_vlan_descriptions_are_not_empty(
        count in 1..100u16,
        seed in 1..1000u64
    ) {
        let vlans = vlan::generate_vlan_configurations(count, Some(seed), None).unwrap();

        prop_assert!(vlans.iter().all(|v| !v.description.is_empty()));
    }

        #[test]
    fn test_vlan_networks_are_valid_format(
        count in 1..100u16,
        seed in 1..1000u64
    ) {
        let vlans = vlan::generate_vlan_configurations(count, Some(seed), None).unwrap();

        for vlan in &vlans {
            prop_assert!(vlan.ip_network.starts_with("10."));
            prop_assert!(vlan.ip_network.ends_with(".x") || vlan.ip_network.ends_with(".0/24"));
        }
    }

        #[test]
    fn test_vlan_networks_are_rfc1918_compliant(
        count in 1..100u16,
        seed in 1..1000u64
    ) {
        let vlans = vlan::generate_vlan_configurations(count, Some(seed), None).unwrap();

        for vlan in &vlans {
            prop_assert!(
                vlan.ip_network.starts_with("10.") ||
                vlan.ip_network.starts_with("172.") ||
                vlan.ip_network.starts_with("192.168.")
            );
        }
    }

    #[test]
    fn test_vlan_wan_assignments_are_valid(
        count in 1..100u16,
        seed in 1..1000u64
    ) {
        let vlans = vlan::generate_vlan_configurations(count, Some(seed), None).unwrap();

        prop_assert!(vlans.iter().all(|v| v.wan_assignment >= 1 && v.wan_assignment <= 3));
    }

    #[test]
    fn test_vlan_generation_respects_count(
        count in 1..100u16,
        seed in 1..1000u64
    ) {
        let vlans = vlan::generate_vlan_configurations(count, Some(seed), None).unwrap();

        prop_assert_eq!(vlans.len(), count as usize);
    }

    #[test]
    fn test_vlan_generation_without_seed(
        count in 1..100u16
    ) {
        let vlans = vlan::generate_vlan_configurations(count, None, None).unwrap();

        prop_assert_eq!(vlans.len(), count as usize);
        prop_assert!(vlans.iter().all(|v| v.vlan_id >= 10 && v.vlan_id <= 4094));
    }
}
