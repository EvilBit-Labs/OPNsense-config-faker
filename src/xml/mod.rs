//! XML processing and generation for OPNsense configurations

pub mod builder;
pub mod engine;
pub mod error;
pub mod generator;
pub mod injection;
pub mod streaming;
pub mod template;

// Re-export key types for convenient usage
pub use builder::OPNsenseConfigBuilder;
pub use engine::XMLEngine;
pub use generator::{ComponentType, XMLGenerator};
pub use injection::XMLInjector;
pub use streaming::StreamingXmlGenerator;
pub use template::{escape_xml_string, XmlTemplate};
