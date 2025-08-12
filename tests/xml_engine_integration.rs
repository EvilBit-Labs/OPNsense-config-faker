//! Integration test demonstrating the new XML processing engine

use opnsense_config_faker::generator::VlanConfig;
use opnsense_config_faker::xml::{OPNsenseConfigBuilder, XMLGenerator};
use opnsense_config_faker::xml::generator::VlanGenerator;

#[test]
fn test_xml_engine_integration() {
    println!("🔧 Testing New XML Processing Engine Integration");
    
    // Create test VLAN configurations
    let vlan_configs = vec![
        VlanConfig::new(100, "10.1.100.x".to_string(), "Development VLAN 100".to_string(), 1).unwrap(),
        VlanConfig::new(200, "10.1.200.x".to_string(), "Production VLAN 200".to_string(), 2).unwrap(),
    ];
    
    // Create VLAN generators
    let mut generators: Vec<Box<dyn XMLGenerator>> = Vec::new();
    for config in vlan_configs {
        generators.push(Box::new(VlanGenerator::new(config)));
    }
    
    // Build configuration using new engine
    let builder = OPNsenseConfigBuilder::new()
        .add_components(generators);
    
    // Generate XML
    let xml_output = builder.build().unwrap();
    
    println!("Generated XML: {}", xml_output);
    
    // Verify the output contains expected VLAN elements
    assert!(xml_output.contains("<vlan>"));
    assert!(xml_output.contains("<vlanid>100</vlanid>"));
    assert!(xml_output.contains("<vlanid>200</vlanid>"));
    assert!(xml_output.contains("Development VLAN 100"));
    assert!(xml_output.contains("Production VLAN 200"));
    assert!(xml_output.contains("<dhcp>"));
    assert!(xml_output.contains("<enable>1</enable>"));
}

#[test]
fn test_xml_engine_memory_estimation() {
    let config = VlanConfig::new(400, "10.1.400.x".to_string(), "Management VLAN 400".to_string(), 3).unwrap();
    let generator = VlanGenerator::new(config);
    
    let memory_estimate = generator.memory_estimate();
    assert!(memory_estimate > 0);
    assert!(memory_estimate < 10000); // Reasonable upper bound
    
    println!("Memory estimate for single VLAN: {} bytes", memory_estimate);
}

#[test]
fn test_xml_engine_multiple_vlans() {
    // Test with multiple VLANs to ensure proper concatenation
    let mut generators: Vec<Box<dyn XMLGenerator>> = Vec::new();
    
    for i in 1..=5 {
        let vlan_id = 100 + i;
        let config = VlanConfig::new(
            vlan_id, 
            format!("10.1.{}.x", vlan_id), 
            format!("Test VLAN {}", vlan_id), 
            ((i % 3) + 1) as u8
        ).unwrap();
        generators.push(Box::new(VlanGenerator::new(config)));
    }
    
    let builder = OPNsenseConfigBuilder::new()
        .add_components(generators);
    
    let xml_output = builder.build().unwrap();
    
    // Should contain all 5 VLANs
    for i in 1..=5 {
        let vlan_id = 100 + i;
        assert!(xml_output.contains(&format!("<vlanid>{}</vlanid>", vlan_id)));
        assert!(xml_output.contains(&format!("Test VLAN {}", vlan_id)));
    }
    
    println!("Successfully generated XML for 5 VLANs");
}