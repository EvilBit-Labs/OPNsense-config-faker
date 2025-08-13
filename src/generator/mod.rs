//! Data generation modules for network configurations

pub mod departments;
pub mod performance;
pub mod vlan;

pub use performance::{PerformanceMetrics, PerformantConfigGenerator};
pub use vlan::{VlanConfig, VlanGenerator};
