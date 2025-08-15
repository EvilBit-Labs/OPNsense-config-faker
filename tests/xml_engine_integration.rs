//! Integration test demonstrating the new XML processing engine

use opnsense_config_faker::generator::VlanConfig;
use opnsense_config_faker::xml::generator::VlanGenerator;
use opnsense_config_faker::xml::{OPNsenseConfigBuilder, XMLGenerator};

#[test]
fn test_xml_engine_integration() {
    println!("🔧 Testing New XML Processing Engine Integration");

    // Create test VLAN configurations
    let vlan_configs = vec![
        VlanConfig::new(
            100,
            "10.1.100.x".to_string(),
            "Development VLAN 100".to_string(),
            1,
        )
        .unwrap(),
        VlanConfig::new(
            200,
            "10.1.200.x".to_string(),
            "Production VLAN 200".to_string(),
            2,
        )
        .unwrap(),
    ];

    // Create VLAN generators
    let mut generators: Vec<Box<dyn XMLGenerator>> = Vec::new();
    for config in vlan_configs {
        generators.push(Box::new(VlanGenerator::new(config)));
    }

    // Build configuration using new engine
    let builder = OPNsenseConfigBuilder::new().add_components(generators);

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
    let config = VlanConfig::new(
        400,
        "10.1.400.x".to_string(),
        "Management VLAN 400".to_string(),
        3,
    )
    .unwrap();
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
            ((i % 3) + 1) as u8,
        )
        .unwrap();
        generators.push(Box::new(VlanGenerator::new(config)));
    }

    let builder = OPNsenseConfigBuilder::new().add_components(generators);

    let xml_output = builder.build().unwrap();

    // Should contain all 5 VLANs
    for i in 1..=5 {
        let vlan_id = 100 + i;
        assert!(xml_output.contains(&format!("<vlanid>{}</vlanid>", vlan_id)));
        assert!(xml_output.contains(&format!("Test VLAN {}", vlan_id)));
    }

    println!("Successfully generated XML for 5 VLANs");
}

#[test]
fn test_enhanced_dhcp_xml_generation() {
    // Create a test VLAN configuration for IT department
    let config = VlanConfig::new(100, "10.1.2.x".to_string(), "IT 100".to_string(), 1).unwrap();

    // Create XML generator with DHCP enabled
    let options = opnsense_config_faker::xml::generator::VlanGeneratorOptions {
        include_dhcp: true,
        ..Default::default()
    };
    let generator = VlanGenerator::with_options(config.clone(), options);

    // Generate XML events
    let events = generator.generate_events().unwrap();

    // Convert events to string for validation
    let mut xml_content = String::new();
    for event in events {
        match event {
            quick_xml::events::Event::Start(start) => {
                xml_content.push_str(&format!(
                    "<{}>",
                    String::from_utf8_lossy(start.name().as_ref())
                ));
            }
            quick_xml::events::Event::End(end) => {
                xml_content.push_str(&format!(
                    "</{}>",
                    String::from_utf8_lossy(end.name().as_ref())
                ));
            }
            quick_xml::events::Event::Text(text) => {
                xml_content.push_str(&String::from_utf8_lossy(&text));
            }
            _ => {}
        }
    }

    println!("Generated XML content:\n{}", xml_content);

    // Verify enhanced DHCP elements are present
    assert!(
        xml_content.contains("<defaultleasetime>86400</defaultleasetime>"),
        "Should contain IT department lease time (24 hours)"
    );
    assert!(
        xml_content.contains("<maxleasetime>172800</maxleasetime>"),
        "Should contain max lease time (48 hours)"
    );
    assert!(
        xml_content.contains("<domain>it.company.local</domain>"),
        "Should contain department-specific domain"
    );
    assert!(
        xml_content.contains("<gateway>10.1.2.1</gateway>"),
        "Should contain gateway"
    );
    assert!(
        xml_content.contains("<dnsserver>10.1.2.1</dnsserver>"),
        "Should contain gateway as DNS server"
    );
    assert!(
        xml_content.contains("<dnsserver>8.8.8.8</dnsserver>"),
        "Should contain Google DNS"
    );
    assert!(
        xml_content.contains("<ntpserver>pool.ntp.org</ntpserver>"),
        "Should contain NTP server"
    );
    assert!(
        xml_content.contains("<staticmap>"),
        "Should contain static reservations"
    );
    assert!(
        xml_content.contains("<hostname>server-it-01</hostname>"),
        "Should contain IT-specific static reservation"
    );
}

#[test]
fn test_enhanced_dhcp_different_departments() {
    // Test Sales department (shorter lease time)
    let sales_config =
        VlanConfig::new(200, "10.1.3.x".to_string(), "Sales 200".to_string(), 1).unwrap();
    let options = opnsense_config_faker::xml::generator::VlanGeneratorOptions {
        include_dhcp: true,
        ..Default::default()
    };
    let sales_generator = VlanGenerator::with_options(sales_config, options.clone());
    let sales_events = sales_generator.generate_events().unwrap();

    let mut sales_xml = String::new();
    for event in sales_events {
        if let quick_xml::events::Event::Text(text) = event {
            sales_xml.push_str(&String::from_utf8_lossy(&text));
        }
    }

    // Sales should have 8-hour lease time (28800 seconds)
    assert!(
        sales_xml.contains("28800"),
        "Sales should have 8-hour lease time"
    );
    assert!(
        sales_xml.contains("sales.company.local"),
        "Sales should have sales domain"
    );

    // Test Security department (different lease time)
    let security_config =
        VlanConfig::new(300, "10.1.4.x".to_string(), "Security 300".to_string(), 1).unwrap();
    let security_generator = VlanGenerator::with_options(security_config, options);
    let security_events = security_generator.generate_events().unwrap();

    let mut security_xml = String::new();
    for event in security_events {
        if let quick_xml::events::Event::Text(text) = event {
            security_xml.push_str(&String::from_utf8_lossy(&text));
        }
    }

    // Security should have 6-hour lease time (21600 seconds)
    assert!(
        security_xml.contains("21600"),
        "Security should have 6-hour lease time"
    );
    assert!(
        security_xml.contains("security.company.local"),
        "Security should have security domain"
    );
}
