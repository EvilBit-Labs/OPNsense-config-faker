#![no_main]

use libfuzzer_sys::fuzz_target;
use opnsense_config_faker::io::csv;
use std::str;

fuzz_target!(|data: &[u8]| {
    // Convert bytes to string, skip if invalid UTF-8
    let csv_str = match str::from_utf8(data) {
        Ok(s) => s,
        Err(_) => return,
    };

    // Skip if the input is too large to be reasonable
    if csv_str.len() > 1024 * 1024 {
        return;
    }

    // Test CSV parsing - we expect this to fail gracefully for invalid input
    let _ = csv::parse_csv_config(csv_str);

    // Test CSV generation with random data
    if let Ok(vlans) =
        opnsense_config_faker::generator::vlan::generate_vlan_configurations(10, None, None)
    {
        let _ = csv::generate_csv(&vlans);
    }
});
