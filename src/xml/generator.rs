//! XML component generators for structured XML generation

use crate::generator::VlanConfig;
use crate::xml::error::XMLResult;
use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Component types for XML generation
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ComponentType {
    /// VLAN configuration component
    Vlan,
    /// Network interface component
    Interface,
    /// DHCP server component
    Dhcp,
    /// NAT rules component
    Nat,
    /// Firewall rules component
    Firewall,
    /// CARP (Common Address Redundancy Protocol) component
    Carp,
    /// RADIUS authentication component
    Radius,
    /// Custom component type
    Custom(String),
}

impl fmt::Display for ComponentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ComponentType::Vlan => write!(f, "VLAN"),
            ComponentType::Interface => write!(f, "Interface"),
            ComponentType::Dhcp => write!(f, "DHCP"),
            ComponentType::Nat => write!(f, "NAT"),
            ComponentType::Firewall => write!(f, "Firewall"),
            ComponentType::Carp => write!(f, "CARP"),
            ComponentType::Radius => write!(f, "RADIUS"),
            ComponentType::Custom(name) => write!(f, "Custom({})", name),
        }
    }
}

/// Trait for XML component generators
pub trait XMLGenerator: Send + Sync {
    /// Get the component type this generator handles
    fn component_type(&self) -> ComponentType;

    /// Generate XML events for this component
    fn generate_events(&self) -> XMLResult<Vec<Event<'static>>>;

    /// Validate component requirements before generation
    fn validate_requirements(&self) -> ValidationResult;

    /// Estimate memory usage for this component (in bytes)
    fn memory_estimate(&self) -> usize;

    /// Get component identifier for debugging
    fn component_id(&self) -> String {
        format!("{}_{}", self.component_type(), uuid::Uuid::new_v4())
    }

    /// Check if this generator supports streaming output
    fn supports_streaming(&self) -> bool {
        false
    }

    /// Generate events in streaming fashion (optional implementation)
    fn generate_streaming_events(
        &self,
        _callback: &mut dyn FnMut(Event<'static>) -> XMLResult<()>,
    ) -> XMLResult<()> {
        // Default implementation falls back to batch generation
        let events = self.generate_events()?;
        for event in events {
            _callback(event)?;
        }
        Ok(())
    }
}

/// Validation result for component requirements
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl ValidationResult {
    /// Create a valid result
    pub fn valid() -> Self {
        Self {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    /// Create an invalid result with errors
    pub fn invalid(errors: Vec<String>) -> Self {
        Self {
            is_valid: false,
            errors,
            warnings: Vec::new(),
        }
    }

    /// Add a warning to the result
    pub fn with_warning(mut self, warning: String) -> Self {
        self.warnings.push(warning);
        self
    }

    /// Add multiple warnings to the result
    pub fn with_warnings(mut self, warnings: Vec<String>) -> Self {
        self.warnings.extend(warnings);
        self
    }

    /// Check if there are any warnings
    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }
}

/// VLAN XML generator implementation
pub struct VlanGenerator {
    config: VlanConfig,
    template_fragment: Option<String>,
    options: VlanGeneratorOptions,
}

/// Options for VLAN XML generation
#[derive(Debug, Clone)]
pub struct VlanGeneratorOptions {
    /// Include DHCP configuration
    pub include_dhcp: bool,
    /// Include firewall rules
    pub include_firewall_rules: bool,
    /// Include NAT rules
    pub include_nat_rules: bool,
    /// Firewall number for rule generation
    pub firewall_number: u16,
    /// OPT interface counter
    pub opt_counter: u16,
}

impl Default for VlanGeneratorOptions {
    fn default() -> Self {
        Self {
            include_dhcp: true,
            include_firewall_rules: false,
            include_nat_rules: false,
            firewall_number: 1,
            opt_counter: 1,
        }
    }
}

impl VlanGenerator {
    /// Create a new VLAN generator
    pub fn new(config: VlanConfig) -> Self {
        Self {
            config,
            template_fragment: None,
            options: VlanGeneratorOptions::default(),
        }
    }

    /// Create a new VLAN generator with options
    pub fn with_options(config: VlanConfig, options: VlanGeneratorOptions) -> Self {
        Self {
            config,
            template_fragment: None,
            options,
        }
    }

    /// Set a custom template fragment
    pub fn with_template_fragment(mut self, fragment: String) -> Self {
        self.template_fragment = Some(fragment);
        self
    }

    /// Generate VLAN XML section events
    fn generate_vlan_events(&self) -> XMLResult<Vec<Event<'static>>> {
        let mut events = Vec::new();

        // Start VLAN element
        let vlan_start = BytesStart::new("vlan");
        events.push(Event::Start(vlan_start));

        // VLAN ID
        events.push(Event::Start(BytesStart::new("vlanid")));
        let vlan_id_text = self.config.vlan_id.to_string();
        events.push(Event::Text(BytesText::new(&vlan_id_text).into_owned()));
        events.push(Event::End(BytesEnd::new("vlanid")));

        // Description
        events.push(Event::Start(BytesStart::new("descr")));
        let description_text = escape_xml_text(&self.config.description);
        events.push(Event::Text(BytesText::new(&description_text).into_owned()));
        events.push(Event::End(BytesEnd::new("descr")));

        // Network configuration
        events.push(Event::Start(BytesStart::new("subnet")));
        events.push(Event::Text(BytesText::new(&self.config.ip_network).into_owned()));
        events.push(Event::End(BytesEnd::new("subnet")));

        // Gateway IP if available
        if let Ok(gateway) = self.config.gateway_ip() {
            events.push(Event::Start(BytesStart::new("gateway")));
            events.push(Event::Text(BytesText::new(&gateway).into_owned()));
            events.push(Event::End(BytesEnd::new("gateway")));
        }

        // DHCP configuration if enabled
        if self.options.include_dhcp {
            events.extend(self.generate_dhcp_events()?);
        }

        // End VLAN element
        events.push(Event::End(BytesEnd::new("vlan")));

        Ok(events)
    }

    /// Generate DHCP server configuration events
    fn generate_dhcp_events(&self) -> XMLResult<Vec<Event<'static>>> {
        let mut events = Vec::new();

        // Start DHCP element
        events.push(Event::Start(BytesStart::new("dhcp")));

        // Enable DHCP
        events.push(Event::Start(BytesStart::new("enable")));
        events.push(Event::Text(BytesText::new("1").into_owned()));
        events.push(Event::End(BytesEnd::new("enable")));

        // DHCP range
        if let (Ok(start), Ok(end)) = (self.config.dhcp_range_start(), self.config.dhcp_range_end()) {
            events.push(Event::Start(BytesStart::new("range")));
            
            events.push(Event::Start(BytesStart::new("from")));
            events.push(Event::Text(BytesText::new(&start).into_owned()));
            events.push(Event::End(BytesEnd::new("from")));

            events.push(Event::Start(BytesStart::new("to")));
            events.push(Event::Text(BytesText::new(&end).into_owned()));
            events.push(Event::End(BytesEnd::new("to")));

            events.push(Event::End(BytesEnd::new("range")));
        }

        // Domain name
        events.push(Event::Start(BytesStart::new("domain")));
        events.push(Event::Text(BytesText::new("local").into_owned()));
        events.push(Event::End(BytesEnd::new("domain")));

        // DNS servers
        events.push(Event::Start(BytesStart::new("dnsserver")));
        if let Ok(gateway) = self.config.gateway_ip() {
            events.push(Event::Text(BytesText::new(&gateway).into_owned()));
        }
        events.push(Event::End(BytesEnd::new("dnsserver")));

        // End DHCP element
        events.push(Event::End(BytesEnd::new("dhcp")));

        Ok(events)
    }
}

impl XMLGenerator for VlanGenerator {
    fn component_type(&self) -> ComponentType {
        ComponentType::Vlan
    }

    fn generate_events(&self) -> XMLResult<Vec<Event<'static>>> {
        if let Some(ref fragment) = self.template_fragment {
            // Use custom template fragment if provided
            self.generate_from_template_fragment(fragment)
        } else {
            // Use default structured generation
            self.generate_vlan_events()
        }
    }

    fn validate_requirements(&self) -> ValidationResult {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // Validate VLAN ID range
        if !(10..=4094).contains(&self.config.vlan_id) {
            errors.push(format!("VLAN ID {} is outside valid range 10-4094", self.config.vlan_id));
        }

        // Validate WAN assignment
        if !(1..=3).contains(&self.config.wan_assignment) {
            errors.push(format!("WAN assignment {} is outside valid range 1-3", self.config.wan_assignment));
        }

        // Validate IP network format
        if !self.config.ip_network.contains('.') {
            errors.push("IP network format is invalid".to_string());
        }

        // Check gateway IP generation
        if self.config.gateway_ip().is_err() {
            warnings.push("Cannot generate gateway IP from network format".to_string());
        }

        // Check DHCP range generation
        if self.options.include_dhcp {
            if self.config.dhcp_range_start().is_err() || self.config.dhcp_range_end().is_err() {
                warnings.push("Cannot generate DHCP range from network format".to_string());
            }
        }

        if errors.is_empty() {
            ValidationResult::valid().with_warnings(warnings)
        } else {
            ValidationResult::invalid(errors).with_warnings(warnings)
        }
    }

    fn memory_estimate(&self) -> usize {
        // Base VLAN configuration: ~512 bytes
        let base_size = 512;
        
        // Additional for DHCP: ~256 bytes
        let dhcp_size = if self.options.include_dhcp { 256 } else { 0 };
        
        // Additional for firewall rules: ~1024 bytes per rule
        let firewall_size = if self.options.include_firewall_rules { 1024 } else { 0 };
        
        // Additional for NAT rules: ~512 bytes per rule
        let nat_size = if self.options.include_nat_rules { 512 } else { 0 };

        base_size + dhcp_size + firewall_size + nat_size
    }

    fn supports_streaming(&self) -> bool {
        true
    }

    fn generate_streaming_events(
        &self,
        callback: &mut dyn FnMut(Event<'static>) -> XMLResult<()>,
    ) -> XMLResult<()> {
        let events = self.generate_events()?;
        for event in events {
            callback(event)?;
        }
        Ok(())
    }
}

impl VlanGenerator {
    /// Generate events from template fragment
    fn generate_from_template_fragment(&self, _fragment: &str) -> XMLResult<Vec<Event<'static>>> {
        // This would parse the template fragment and substitute values
        // For now, fall back to structured generation
        self.generate_vlan_events()
    }
}

/// Escape XML text content
fn escape_xml_text(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
        // Handle German umlauts
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
    fn test_component_type_display() {
        assert_eq!(ComponentType::Vlan.to_string(), "VLAN");
        assert_eq!(ComponentType::Custom("Test".to_string()).to_string(), "Custom(Test)");
    }

    #[test]
    fn test_validation_result() {
        let valid = ValidationResult::valid();
        assert!(valid.is_valid);
        assert!(valid.errors.is_empty());

        let invalid = ValidationResult::invalid(vec!["Error 1".to_string()]);
        assert!(!invalid.is_valid);
        assert_eq!(invalid.errors.len(), 1);
    }

    #[test]
    fn test_vlan_generator_creation() {
        let config = VlanConfig::new(100, "10.1.2.x".to_string(), "Test VLAN".to_string(), 1).unwrap();
        let generator = VlanGenerator::new(config);
        assert_eq!(generator.component_type(), ComponentType::Vlan);
    }

    #[test]
    fn test_vlan_generator_validation() {
        let config = VlanConfig::new(100, "10.1.2.x".to_string(), "Test VLAN".to_string(), 1).unwrap();
        let generator = VlanGenerator::new(config);
        let validation = generator.validate_requirements();
        assert!(validation.is_valid);
    }

    #[test]
    fn test_vlan_generator_validation_errors() {
        let config = VlanConfig::new(5000, "invalid".to_string(), "Test VLAN".to_string(), 5).unwrap_or_else(|_| {
            // Create a config that will pass initial creation but fail XML validation
            VlanConfig::new(100, "10.1.2.x".to_string(), "Test VLAN".to_string(), 1).unwrap()
        });
        
        // Manually create an invalid config for testing
        let mut generator = VlanGenerator::new(config);
        generator.config.vlan_id = 5000; // Invalid VLAN ID
        generator.config.wan_assignment = 5; // Invalid WAN assignment
        
        let validation = generator.validate_requirements();
        assert!(!validation.is_valid);
        assert!(!validation.errors.is_empty());
    }

    #[test]
    fn test_vlan_generator_memory_estimate() {
        let config = VlanConfig::new(100, "10.1.2.x".to_string(), "Test VLAN".to_string(), 1).unwrap();
        let generator = VlanGenerator::new(config);
        let estimate = generator.memory_estimate();
        assert!(estimate > 0);
        assert!(estimate < 10000); // Reasonable upper bound
    }

    #[test]
    fn test_vlan_generator_events_generation() {
        let config = VlanConfig::new(100, "10.1.2.x".to_string(), "Test VLAN".to_string(), 1).unwrap();
        let generator = VlanGenerator::new(config);
        let events = generator.generate_events().unwrap();
        assert!(!events.is_empty());
        
        // Check that we have start and end events
        let has_start = events.iter().any(|e| matches!(e, Event::Start(_)));
        let has_end = events.iter().any(|e| matches!(e, Event::End(_)));
        assert!(has_start);
        assert!(has_end);
    }

    #[test]
    fn test_escape_xml_text() {
        assert_eq!(escape_xml_text("Hello & World"), "Hello &amp; World");
        assert_eq!(escape_xml_text("<test>"), "&lt;test&gt;");
        assert_eq!(escape_xml_text("Größe"), "Groesse");
        assert_eq!(escape_xml_text("Test \"quote\""), "Test &quot;quote&quot;");
    }

    #[test]
    fn test_vlan_generator_with_options() {
        let config = VlanConfig::new(100, "10.1.2.x".to_string(), "Test VLAN".to_string(), 1).unwrap();
        let options = VlanGeneratorOptions {
            include_dhcp: true,
            include_firewall_rules: true,
            ..Default::default()
        };
        let generator = VlanGenerator::with_options(config, options);
        
        let memory_estimate = generator.memory_estimate();
        assert!(memory_estimate > 512); // Should be larger with DHCP and firewall
    }

    #[test]
    fn test_vlan_generator_supports_streaming() {
        let config = VlanConfig::new(100, "10.1.2.x".to_string(), "Test VLAN".to_string(), 1).unwrap();
        let generator = VlanGenerator::new(config);
        assert!(generator.supports_streaming());
    }
}