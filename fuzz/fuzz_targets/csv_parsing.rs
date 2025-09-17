#![no_main]

use libfuzzer_sys::fuzz_target;
use std::fs;
use std::str;
use tempfile::NamedTempFile;

use opnsense_config_faker::generator::vlan::generate_vlan_configurations;
use opnsense_config_faker::io::csv::{read_csv_validated, write_csv_streaming};

fuzz_target!(|data: &[u8]| {
    // Convert bytes to string; skip invalid UTF-8 inputs
    let csv_str = match str::from_utf8(data) {
        Ok(s) => s,
        Err(_) => return,
    };

    // Bound input size to keep runs reasonable
    if csv_str.len() > 1024 * 1024 {
        return;
    }

    // 1) CSV parsing path (file-based API with validation)
    if let Ok(temp) = NamedTempFile::new() {
        let _ = fs::write(temp.path(), csv_str);
        let _ = read_csv_validated(temp.path());
    }

    // 2) CSV generation path (streaming writer + read back)
    // Use a fixed seed for deterministic fuzzing and cast count to u16 explicitly
    let count: u16 = 10;
    let seed: Option<u64> = Some(42); // Fixed seed for reproducible fuzzing
    if let Ok(vlans) = generate_vlan_configurations(count, seed, None) {
        if let Ok(temp) = NamedTempFile::new() {
            let _ = write_csv_streaming(vlans.into_iter(), temp.path());
            if let Ok(content) = fs::read_to_string(temp.path()) {
                // Touch the header to ensure it's present; ignore mismatches
                let _ = content
                    .lines()
                    .next()
                    .map(|l| l == "VLAN,IP Range,Beschreibung,WAN");
            }
        }
    }
});
