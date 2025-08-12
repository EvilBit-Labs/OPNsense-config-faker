//! OPNsense Config Faker Library
//!
//! A flexible tool for generating realistic network configuration test data for OPNsense.
//! This library provides core functionality for generating VLAN configurations, CSV data,
//! and complete OPNsense XML configurations with realistic test data.
//!
//! # Features
//!
//! - Generate realistic VLAN configurations with unique IDs (10-4094 range)
//! - Create RFC 1918 compliant private IP networks
//! - Export configurations as CSV or complete OPNsense XML
//! - Ensure data consistency and uniqueness across generated configurations
//! - Command-line interface with progress indicators and rich output
//!
//! # Example
//!
//! ```rust,no_run
//! use opnsense_config_faker::generator::VlanConfig;
//! use opnsense_config_faker::generator::vlan::generate_vlan_configurations;
//!
//! // Generate 10 VLAN configurations
//! let configs = generate_vlan_configurations(10, None, None)?;
//!
//! // Each config has unique VLAN ID and IP network
//! for config in &configs {
//!     println!("VLAN {}: {}", config.vlan_id, config.ip_network);
//! }
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

pub mod cli;
pub mod generator;
pub mod io;
pub mod model;
pub mod utils;
pub mod validate;
pub mod xml;

// Re-export commonly used types
pub use crate::generator::{VlanConfig, VlanGenerator};
pub use crate::model::error::ConfigError;

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Result type alias for this crate
pub type Result<T> = std::result::Result<T, ConfigError>;
