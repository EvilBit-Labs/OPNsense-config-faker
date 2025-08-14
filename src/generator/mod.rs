//! Data generation modules for network configurations

pub mod departments;
pub mod firewall;
pub mod performance;
pub mod vlan;

pub use firewall::{generate_firewall_rules, FirewallComplexity, FirewallGenerator, FirewallRule};
pub use performance::{PerformanceMetrics, PerformantConfigGenerator};
pub use vlan::{VlanConfig, VlanGenerator};
