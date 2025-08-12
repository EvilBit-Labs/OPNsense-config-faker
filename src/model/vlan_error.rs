//! VLAN-specific error types

use thiserror::Error;

/// VLAN-specific error types for enhanced error handling
#[derive(Debug, Error)]
pub enum VlanError {
    /// VLAN ID is outside the valid range (10-4094)
    #[error("VLAN ID {0} is outside valid range 10-4094")]
    InvalidVlanId(u16),

    /// Network is not RFC 1918 compliant
    #[error("Network {0} is not RFC 1918 compliant")]
    NonRfc1918Network(String),

    /// Network parsing failed
    #[error("Network parsing failed: {0}")]
    NetworkParsing(String),

    /// VLAN ID pool exhausted (all unique IDs used)
    #[error("VLAN ID pool exhausted - no more unique IDs available")]
    VlanIdExhausted,

    /// Network pool exhausted (all unique networks used)
    #[error("Network pool exhausted - no more unique networks available")]
    NetworkExhausted,

    /// WAN assignment is outside valid range (1-3)
    #[error("WAN assignment {0} is outside valid range 1-3")]
    InvalidWanAssignment(u8),

    /// Department name is invalid or empty
    #[error("Department name is invalid: {0}")]
    InvalidDepartment(String),

    /// Configuration validation failed
    #[error("Configuration validation failed: {0}")]
    ValidationFailed(String),
}

impl VlanError {
    /// Create a validation error with a custom message
    pub fn validation<S: Into<String>>(message: S) -> Self {
        Self::ValidationFailed(message.into())
    }

    /// Create a network parsing error with a custom message
    pub fn network_parsing<S: Into<String>>(message: S) -> Self {
        Self::NetworkParsing(message.into())
    }

    /// Create an invalid department error with a custom message
    pub fn invalid_department<S: Into<String>>(message: S) -> Self {
        Self::InvalidDepartment(message.into())
    }
}

/// Result type alias for VLAN operations
pub type VlanResult<T> = std::result::Result<T, VlanError>;
