//! XML processing error types for enhanced diagnostics

use thiserror::Error;

/// Specialized XML processing error types
#[derive(Debug, Error)]
pub enum XMLError {
    /// XML parsing error from quick-xml
    #[error("XML parsing failed: {0}")]
    Parsing(#[from] quick_xml::Error),

    /// Template loading error
    #[error("Template loading failed: {source}")]
    TemplateLoading {
        #[from]
        source: std::io::Error,
    },

    /// Injection point not found in template
    #[error("Injection point not found: {selector}")]
    InjectionPointNotFound { selector: String },

    /// Schema validation failure
    #[error("Schema validation failed: {errors:?}")]
    SchemaValidation { errors: Vec<ValidationError> },

    /// Memory limit exceeded during processing
    #[error("Memory limit exceeded: {current}MB > {limit}MB")]
    MemoryLimitExceeded { current: usize, limit: usize },

    /// Namespace processing error
    #[error("XML namespace error: {message}")]
    Namespace { message: String },

    /// XML generation error from component
    #[error("XML generation failed: {component} - {reason}")]
    Generation { component: String, reason: String },

    /// Invalid XML structure
    #[error("Invalid XML structure: {message}")]
    InvalidStructure { message: String },
}

/// Validation error details
#[derive(Debug, Clone)]
pub struct ValidationError {
    pub path: String,
    pub message: String,
    pub severity: ValidationSeverity,
}

/// Validation error severity levels
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationSeverity {
    Error,
    Warning,
    Info,
}

impl XMLError {
    /// Create a new injection point not found error
    pub fn injection_point_not_found<S: Into<String>>(selector: S) -> Self {
        Self::InjectionPointNotFound {
            selector: selector.into(),
        }
    }

    /// Create a new schema validation error
    pub fn schema_validation(errors: Vec<ValidationError>) -> Self {
        Self::SchemaValidation { errors }
    }

    /// Create a new memory limit exceeded error
    pub fn memory_limit_exceeded(current: usize, limit: usize) -> Self {
        Self::MemoryLimitExceeded { current, limit }
    }

    /// Create a new namespace error
    pub fn namespace<S: Into<String>>(message: S) -> Self {
        Self::Namespace {
            message: message.into(),
        }
    }

    /// Create a new generation error
    pub fn generation<S: Into<String>, R: Into<String>>(component: S, reason: R) -> Self {
        Self::Generation {
            component: component.into(),
            reason: reason.into(),
        }
    }

    /// Create a new invalid structure error
    pub fn invalid_structure<S: Into<String>>(message: S) -> Self {
        Self::InvalidStructure {
            message: message.into(),
        }
    }
}

impl ValidationError {
    /// Create a new validation error
    pub fn new<P: Into<String>, M: Into<String>>(
        path: P,
        message: M,
        severity: ValidationSeverity,
    ) -> Self {
        Self {
            path: path.into(),
            message: message.into(),
            severity,
        }
    }

    /// Create an error-level validation error
    pub fn error<P: Into<String>, M: Into<String>>(path: P, message: M) -> Self {
        Self::new(path, message, ValidationSeverity::Error)
    }

    /// Create a warning-level validation error
    pub fn warning<P: Into<String>, M: Into<String>>(path: P, message: M) -> Self {
        Self::new(path, message, ValidationSeverity::Warning)
    }

    /// Create an info-level validation error
    pub fn info<P: Into<String>, M: Into<String>>(path: P, message: M) -> Self {
        Self::new(path, message, ValidationSeverity::Info)
    }
}

/// Type alias for XML processing results
pub type XMLResult<T> = Result<T, XMLError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xml_error_creation() {
        let error = XMLError::injection_point_not_found("//vlan[@id='100']");
        assert!(matches!(error, XMLError::InjectionPointNotFound { .. }));
    }

    #[test]
    fn test_validation_error_creation() {
        let error = ValidationError::error("/opnsense/vlan", "Invalid VLAN ID");
        assert_eq!(error.severity, ValidationSeverity::Error);
        assert_eq!(error.path, "/opnsense/vlan");
        assert_eq!(error.message, "Invalid VLAN ID");
    }

    #[test]
    fn test_memory_limit_error() {
        let error = XMLError::memory_limit_exceeded(50, 32);
        assert!(matches!(error, XMLError::MemoryLimitExceeded { .. }));
    }

    #[test]
    fn test_generation_error() {
        let error = XMLError::generation("VLANGenerator", "Invalid VLAN ID range");
        assert!(matches!(error, XMLError::Generation { .. }));
    }
}