//! Error types for OPNsense Config Faker

use thiserror::Error;

/// Main error type for the OPNsense Config Faker
#[derive(Debug, Error)]
pub enum ConfigError {
    /// I/O operation failed
    #[error("I/O operation failed: {0}")]
    Io(#[from] std::io::Error),

    /// CSV operation failed
    #[error("CSV operation failed: {0}")]
    Csv(#[from] csv::Error),

    /// XML processing failed
    #[error("XML processing failed: {0}")]
    Xml(#[from] quick_xml::Error),

    /// JSON serialization/deserialization failed
    #[error("JSON operation failed: {0}")]
    Json(#[from] serde_json::Error),

    /// Network address parsing failed
    #[error("Network address parsing failed: {0}")]
    Network(#[from] ipnet::AddrParseError),

    /// VLAN configuration generation failed
    #[error("VLAN generation failed: {message}")]
    VlanGeneration { message: String },

    /// Validation error
    #[error("Validation error: {message}")]
    Validation { message: String },

    /// XML template processing failed
    #[error("XML template error: {message}")]
    XmlTemplate { message: String },

    /// Configuration file not found
    #[error("Configuration file not found: {path}")]
    ConfigNotFound { path: String },

    /// Invalid configuration parameter
    #[error("Invalid parameter '{parameter}': {reason}")]
    InvalidParameter { parameter: String, reason: String },

    /// Resource exhaustion (e.g., ran out of unique VLAN IDs)
    #[error("Resource exhaustion: {resource}")]
    ResourceExhausted { resource: String },

    /// Generic configuration error
    #[error("Configuration error: {message}")]
    Config { message: String },
}

impl ConfigError {
    /// Create a new validation error
    pub fn validation<S: Into<String>>(message: S) -> Self {
        Self::Validation {
            message: message.into(),
        }
    }

    /// Create a new VLAN generation error
    pub fn vlan_generation<S: Into<String>>(message: S) -> Self {
        Self::VlanGeneration {
            message: message.into(),
        }
    }

    /// Create a new XML template error
    pub fn xml_template<S: Into<String>>(message: S) -> Self {
        Self::XmlTemplate {
            message: message.into(),
        }
    }

    /// Create a new configuration error
    pub fn config<S: Into<String>>(message: S) -> Self {
        Self::Config {
            message: message.into(),
        }
    }

    /// Create a new invalid parameter error
    pub fn invalid_parameter<S: Into<String>, R: Into<String>>(parameter: S, reason: R) -> Self {
        Self::InvalidParameter {
            parameter: parameter.into(),
            reason: reason.into(),
        }
    }

    /// Create a new resource exhausted error
    pub fn resource_exhausted<S: Into<String>>(resource: S) -> Self {
        Self::ResourceExhausted {
            resource: resource.into(),
        }
    }
}