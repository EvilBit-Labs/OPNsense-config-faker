//! XML template processing for OPNsense configurations

use crate::Result;
use crate::generator::VlanConfig;
use crate::model::ConfigError;

/// XML template processor for OPNsense configurations
pub struct XmlTemplate {
    base_content: String,
}

impl XmlTemplate {
    /// Create a new XML template from base content
    pub fn new(base_content: String) -> Result<Self> {
        // Basic validation that this looks like XML
        if !base_content.trim_start().starts_with("<?xml")
            && !base_content.trim_start().starts_with('<')
        {
            return Err(ConfigError::xml_template(
                "Base content does not appear to be valid XML",
            ));
        }

        Ok(Self { base_content })
    }

    /// Apply a VLAN configuration to generate an XML configuration
    pub fn apply_configuration(
        &self,
        config: &VlanConfig,
        firewall_nr: u16,
        opt_counter: u16,
    ) -> Result<String> {
        // This is a placeholder implementation
        // In the full implementation, this would use the OPNsense XML generation
        // logic from the Python version, adapted to Rust

        let mut result = self.base_content.clone();

        // Replace placeholder values — all user-derived values are XML-escaped
        // to prevent XML injection (CWE-91) from crafted CSV input
        result = result.replace("{{VLAN_ID}}", &config.vlan_id.to_string());
        result = result.replace("{{IP_NETWORK}}", &escape_xml_string(&config.ip_network));
        result = result.replace("{{DESCRIPTION}}", &escape_xml_string(&config.description));
        result = result.replace("{{WAN_ASSIGNMENT}}", &config.wan_assignment.to_string());
        result = result.replace("{{FIREWALL_NR}}", &firewall_nr.to_string());
        result = result.replace("{{OPT_COUNTER}}", &opt_counter.to_string());

        // Add gateway IP if possible
        if let Ok(gateway) = config.gateway_ip() {
            result = result.replace("{{GATEWAY_IP}}", &escape_xml_string(&gateway));
        }

        // Add DHCP range if possible
        if let Ok(dhcp_start) = config.dhcp_range_start() {
            result = result.replace("{{DHCP_START}}", &escape_xml_string(&dhcp_start));
        }

        if let Ok(dhcp_end) = config.dhcp_range_end() {
            result = result.replace("{{DHCP_END}}", &escape_xml_string(&dhcp_end));
        }

        Ok(result)
    }
}

/// Escape XML special characters in a string
pub fn escape_xml_string(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
        // Handle German umlauts as in Python version
        .replace('ä', "ae")
        .replace('ö', "oe")
        .replace('ü', "ue")
        .replace('Ä', "Ae")
        .replace('Ö', "Oe")
        .replace('Ü', "Ue")
        .replace('ß', "ss")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xml_template_creation() {
        let xml_content = r#"<?xml version="1.0"?>
<opnsense>
    <vlan id="{{VLAN_ID}}">{{DESCRIPTION}}</vlan>
</opnsense>"#;

        let template = XmlTemplate::new(xml_content.to_string()).unwrap();
        assert!(!template.base_content.is_empty());
    }

    #[test]
    fn test_xml_template_invalid_content() {
        let invalid_content = "This is not XML";
        let result = XmlTemplate::new(invalid_content.to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_configuration() {
        let xml_content = r#"<?xml version="1.0"?>
<opnsense>
    <vlan id="{{VLAN_ID}}">{{DESCRIPTION}}</vlan>
    <network>{{IP_NETWORK}}</network>
    <gateway>{{GATEWAY_IP}}</gateway>
</opnsense>"#;

        let template = XmlTemplate::new(xml_content.to_string()).unwrap();
        let config =
            VlanConfig::new(100, "10.1.2.x".to_string(), "Test VLAN 100".to_string(), 1).unwrap();

        let result = template.apply_configuration(&config, 1, 6).unwrap();

        assert!(result.contains(r#"<vlan id="100">Test VLAN 100</vlan>"#));
        assert!(result.contains("<network>10.1.2.x</network>"));
        assert!(result.contains("<gateway>10.1.2.1</gateway>"));
    }

    #[test]
    fn test_escape_xml_string() {
        assert_eq!(escape_xml_string("Hello & World"), "Hello &amp; World");
        assert_eq!(escape_xml_string("<tag>"), "&lt;tag&gt;");
        assert_eq!(escape_xml_string("\"quoted\""), "&quot;quoted&quot;");
        assert_eq!(escape_xml_string("Größe"), "Groesse");
        assert_eq!(escape_xml_string("Mädchen"), "Maedchen");
    }
}
