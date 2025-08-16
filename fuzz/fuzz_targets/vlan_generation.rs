#![no_main]

use libfuzzer_sys::fuzz_target;
use opnsense_config_faker::generator::vlan;

fuzz_target!(|data: &[u8]| {
    // Use the fuzz input to generate parameters
    if data.len() < 8 {
        return;
    }

    // Extract parameters from fuzz input
    let count = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
    let base_id = u16::from_le_bytes([data[4], data[5]]);

    // Limit the range to reasonable values to avoid timeouts
    let count = count % 1000; // Max 1000 VLANs
    let base_id = base_id % 4000; // Max base ID 4000

    // Test VLAN generation with fuzzed parameters
    let _ = vlan::generate_vlan_configurations(count, Some(base_id), None);

    // Test with different network bases
    if let Ok(network) = "192.168.0.0/24".parse() {
        let _ = vlan::generate_vlan_configurations(count, Some(base_id), Some(network));
    }
});
