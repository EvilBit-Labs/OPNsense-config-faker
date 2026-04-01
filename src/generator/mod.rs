//! Data generation modules for network configurations

pub mod departments;
pub mod firewall;
pub mod nat;
pub mod performance;
pub mod vlan;
pub mod vpn;

pub use firewall::{FirewallComplexity, FirewallGenerator, FirewallRule, generate_firewall_rules};
pub use nat::{NatGenerator, NatMapping, NatRuleType, generate_nat_mappings};
pub use performance::{PerformanceMetrics, PerformantConfigGenerator};
pub use vlan::{VlanConfig, VlanGenerator};
pub use vpn::{VpnConfig, VpnGenerator, VpnType, generate_vpn_configurations};
