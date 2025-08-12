//! Data models and structures for OPNsense configuration generation

pub mod error;
pub mod vlan_error;

pub use error::ConfigError;
pub use vlan_error::{VlanError, VlanResult};
