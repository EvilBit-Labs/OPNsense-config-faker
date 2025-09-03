//! Data generation modules for network configurations

pub mod departments;
pub mod firewall;
pub mod nat;
pub mod performance;
pub mod vlan;
pub mod vpn;

pub use firewall::{generate_firewall_rules, FirewallComplexity, FirewallGenerator, FirewallRule};
pub use nat::{generate_nat_mappings, NatGenerator, NatMapping, NatRuleType};
pub use performance::{PerformanceMetrics, PerformantConfigGenerator};
pub use vlan::{VlanConfig, VlanGenerator};
pub use vpn::{generate_vpn_configurations, VpnConfig, VpnGenerator, VpnType};
