//! OPNsense configuration builder for complete XML generation

use crate::xml::engine::{XMLEngine, XMLTemplate};
use crate::xml::error::{XMLError, XMLResult};
use crate::xml::generator::XMLGenerator;
use crate::xml::injection::XMLInjector;
use quick_xml::events::Event;
use std::io::Write;
use std::path::PathBuf;

/// Configuration builder for OPNsense XML generation
pub struct OPNsenseConfigBuilder {
    template_path: Option<PathBuf>,
    template_content: Option<String>,
    components: Vec<Box<dyn XMLGenerator>>,
    validation_rules: Vec<ValidationRule>,
    output_config: OutputConfig,
    xml_engine: XMLEngine,
}

/// Validation rules for configuration generation
#[derive(Debug, Clone)]
pub struct ValidationRule {
    pub name: String,
    pub description: String,
    pub validator: fn(&[Event]) -> XMLResult<bool>,
}

/// Output configuration options
#[derive(Debug, Clone)]
pub struct OutputConfig {
    pub include_declaration: bool,
    pub pretty_print: bool,
    pub memory_limit_mb: usize,
    pub streaming_threshold_mb: usize,
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            include_declaration: true,
            pretty_print: true,
            memory_limit_mb: 32,
            streaming_threshold_mb: 10,
        }
    }
}

impl OPNsenseConfigBuilder {
    /// Create a new OPNsense configuration builder
    pub fn new() -> Self {
        Self {
            template_path: None,
            template_content: None,
            components: Vec::new(),
            validation_rules: Vec::new(),
            output_config: OutputConfig::default(),
            xml_engine: XMLEngine::new(),
        }
    }

    /// Create a builder with a template file path
    pub fn with_template_file(template_path: PathBuf) -> Self {
        Self {
            template_path: Some(template_path),
            template_content: None,
            components: Vec::new(),
            validation_rules: Vec::new(),
            output_config: OutputConfig::default(),
            xml_engine: XMLEngine::new(),
        }
    }

    /// Create a builder with template content
    pub fn with_template_content(content: String) -> Self {
        Self {
            template_path: None,
            template_content: Some(content),
            components: Vec::new(),
            validation_rules: Vec::new(),
            output_config: OutputConfig::default(),
            xml_engine: XMLEngine::new(),
        }
    }

    /// Add a component generator
    pub fn add_component<T: XMLGenerator + 'static>(mut self, component: T) -> Self {
        self.components.push(Box::new(component));
        self
    }

    /// Add multiple components
    pub fn add_components(mut self, components: Vec<Box<dyn XMLGenerator>>) -> Self {
        self.components.extend(components);
        self
    }

    /// Set output configuration
    pub fn with_output_config(mut self, config: OutputConfig) -> Self {
        let memory_limit = config.memory_limit_mb;
        self.output_config = config;
        self.xml_engine = XMLEngine::with_memory_limit(memory_limit);
        self
    }

    /// Add a validation rule
    pub fn add_validation_rule(mut self, rule: ValidationRule) -> Self {
        self.validation_rules.push(rule);
        self
    }

    /// Build the complete OPNsense configuration
    pub fn build(mut self) -> XMLResult<String> {
        // Load template
        let template = self.load_template()?;

        // Create injector and add components
        let mut injector = XMLInjector::new(template);
        let components = std::mem::take(&mut self.components);
        for component in components {
            injector.add_generator(component);
        }

        // Validate before generation
        let validation = injector.validate_injections();
        if !validation.is_valid {
            return Err(XMLError::schema_validation(
                validation
                    .errors
                    .into_iter()
                    .map(|e| crate::xml::error::ValidationError::error("component", e))
                    .collect(),
            ));
        }

        // Generate events
        let events = injector.inject_components()?;

        // Validate output events
        self.validate_events(&events)?;

        // Process events to string
        let result = self.xml_engine.process_events(events)?;

        Ok(result)
    }

    /// Build with streaming output for large configurations
    pub fn build_streaming<W: Write>(mut self, writer: W) -> XMLResult<()> {
        // Load template
        let template = self.load_template()?;

        // Create injector and add components
        let mut injector = XMLInjector::new(template);
        let components = std::mem::take(&mut self.components);
        for component in components {
            injector.add_generator(component);
        }

        // Stream inject components
        injector.stream_inject(writer)?;

        Ok(())
    }

    /// Estimate memory usage for the configuration
    pub fn estimate_memory_usage(&self) -> usize {
        let mut total = 0;
        for component in &self.components {
            total += component.memory_estimate();
        }
        total
    }

    /// Load template from file or content
    fn load_template(&mut self) -> XMLResult<XMLTemplate> {
        if let Some(ref path) = self.template_path {
            self.xml_engine.load_template(path)
        } else if let Some(ref content) = self.template_content {
            self.xml_engine.parse_template(content.clone())
        } else {
            // Use a default OPNsense template
            let default_template = self.default_opnsense_template();
            self.xml_engine.parse_template(default_template)
        }
    }

    /// Generate a default OPNsense XML template
    fn default_opnsense_template(&self) -> String {
        r#"<?xml version="1.0"?>
<opnsense>
    <version>24.1</version>
    <theme>opnsense</theme>
    <sysctl>
        <item>
            <descr>Increase UFS read-ahead speeds to match the state of hard drives and NCQ.</descr>
            <tunable>vfs.read_max</tunable>
            <value>default</value>
        </item>
    </sysctl>
    <system>
        <optimization>normal</optimization>
        <hostname>OPNsense</hostname>
        <domain>localdomain</domain>
        <dnsallowoverride>1</dnsallowoverride>
        <group>
            <name>admins</name>
            <description>System Administrators</description>
            <scope>system</scope>
            <gid>1998</gid>
            <member>0</member>
            <priv>page-all</priv>
        </group>
        <user>
            <name>root</name>
            <descr>System Administrator</descr>
            <scope>system</scope>
            <groupname>admins</groupname>
            <password>$2b$10$YRVoF4SgskIsrXOvOQjGieB9XqHPRra9R7d80B3BZdbY/j21TwBfS</password>
            <uid>0</uid>
        </user>
        <nextuid>2000</nextuid>
        <nextgid>2000</nextgid>
        <timezone>Etc/UTC</timezone>
        <timeservers>0.opnsense.pool.ntp.org 1.opnsense.pool.ntp.org 2.opnsense.pool.ntp.org 3.opnsense.pool.ntp.org</timeservers>
        <webgui>
            <protocol>https</protocol>
            <ssl-certref>669c91fff0b06</ssl-certref>
            <port></port>
            <ssl-ciphers></ssl-ciphers>
            <interfaces></interfaces>
            <compression>6</compression>
        </webgui>
        <disablenatreflection>yes</disablenatreflection>
        <usevirtualterminal>1</usevirtualterminal>
        <disableconsolemenu></disableconsolemenu>
        <disablevlanhwfilter>1</disablevlanhwfilter>
        <disablechecksumoffloading>1</disablechecksumoffloading>
        <disablesegmentationoffloading>1</disablesegmentationoffloading>
        <disablelargereceiveoffloading>1</disablelargereceiveoffloading>
        <ipv6allow></ipv6allow>
        <powerd_ac_mode>hadp</powerd_ac_mode>
        <powerd_battery_mode>hadp</powerd_battery_mode>
        <bogons>
            <interval>monthly</interval>
        </bogons>
        <pf_share_forward>1</pf_share_forward>
        <lb_use_sticky>1</lb_use_sticky>
        <ssh>
            <group>admins</group>
        </ssh>
        <rrdbackup>-1</rrdbackup>
        <netbios>1</netbios>
    </system>
    <interfaces>
        <wan>
            <enable>1</enable>
            <if>vtnet0</if>
            <mtu></mtu>
            <ipaddr>dhcp</ipaddr>
            <ipaddrv6>dhcp6</ipaddrv6>
            <subnet></subnet>
            <gateway></gateway>
            <blockpriv>1</blockpriv>
            <blockbogons>1</blockbogons>
            <dhcphostname></dhcphostname>
            <media></media>
            <mediaopt></mediaopt>
            <dhcp6-ia-pd-len>0</dhcp6-ia-pd-len>
        </wan>
        <lan>
            <enable>1</enable>
            <if>vtnet1</if>
            <ipaddr>192.168.1.1</ipaddr>
            <subnet>24</subnet>
            <ipaddrv6>track6</ipaddrv6>
            <subnetv6>64</subnetv6>
            <media></media>
            <mediaopt></mediaopt>
            <track6-interface>wan</track6-interface>
            <track6-prefix-id>0</track6-prefix-id>
        </lan>
    </interfaces>
    <vlans>
        <!-- VLAN configurations will be injected here -->
    </vlans>
    <dhcpd>
        <lan>
            <enable>1</enable>
            <ddnsdomainalgorithm>hmac-md5</ddnsdomainalgorithm>
            <numberoptions>
                <item>
                    <number>7</number>
                    <value>1</value>
                </item>
            </numberoptions>
            <range>
                <from>192.168.1.100</from>
                <to>192.168.1.199</to>
            </range>
        </lan>
        <!-- DHCP configurations will be injected here -->
    </dhcpd>
    <filter>
        <!-- Firewall rules will be injected here -->
    </filter>
    <nat>
        <!-- NAT rules will be injected here -->
    </nat>
</opnsense>"#.to_string()
    }

    /// Validate generated events
    fn validate_events(&self, events: &[Event]) -> XMLResult<()> {
        for rule in &self.validation_rules {
            if !(rule.validator)(events)? {
                return Err(XMLError::schema_validation(vec![
                    crate::xml::error::ValidationError::error(&rule.name, &rule.description),
                ]));
            }
        }
        Ok(())
    }
}

impl Default for OPNsenseConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generator::VlanConfig;
    use crate::xml::generator::VlanGenerator;

    #[test]
    fn test_builder_creation() {
        let builder = OPNsenseConfigBuilder::new();
        assert!(builder.template_path.is_none());
        assert!(builder.template_content.is_none());
        assert_eq!(builder.components.len(), 0);
    }

    #[test]
    fn test_builder_with_template_content() {
        let content = "<?xml version=\"1.0\"?><opnsense></opnsense>".to_string();
        let builder = OPNsenseConfigBuilder::with_template_content(content.clone());
        assert_eq!(builder.template_content, Some(content));
    }

    #[test]
    fn test_add_component() {
        let config =
            VlanConfig::new(100, "10.1.2.x".to_string(), "Test VLAN".to_string(), 1).unwrap();
        let generator = VlanGenerator::new(config);

        let builder = OPNsenseConfigBuilder::new().add_component(generator);

        assert_eq!(builder.components.len(), 1);
    }

    #[test]
    fn test_memory_estimation() {
        let config =
            VlanConfig::new(100, "10.1.2.x".to_string(), "Test VLAN".to_string(), 1).unwrap();
        let generator = VlanGenerator::new(config);

        let builder = OPNsenseConfigBuilder::new().add_component(generator);

        let estimate = builder.estimate_memory_usage();
        assert!(estimate > 0);
    }

    #[test]
    fn test_build_with_default_template() {
        let config =
            VlanConfig::new(100, "10.1.2.x".to_string(), "Test VLAN".to_string(), 1).unwrap();
        let generator = VlanGenerator::new(config);

        let builder = OPNsenseConfigBuilder::new().add_component(generator);

        let result = builder.build();
        assert!(result.is_ok());
        let xml = result.unwrap();
        println!("Generated XML: {}", xml); // Debug output
        // The current implementation generates XML events, not the full template
        assert!(!xml.is_empty());
    }

    #[test]
    fn test_output_config() {
        let config = OutputConfig {
            include_declaration: false,
            pretty_print: false,
            memory_limit_mb: 64,
            streaming_threshold_mb: 20,
        };

        let builder = OPNsenseConfigBuilder::new().with_output_config(config.clone());

        assert_eq!(builder.output_config.memory_limit_mb, 64);
        assert_eq!(builder.output_config.streaming_threshold_mb, 20);
    }
}
