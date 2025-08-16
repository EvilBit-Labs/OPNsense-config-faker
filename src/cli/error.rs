//! CLI-specific error types for enhanced error handling

use thiserror::Error;

/// CLI-specific error types for command-line interface operations
#[derive(Debug, Error)]
pub enum CliError {
    /// Invalid command-line argument
    #[error("Invalid command-line argument: {0}")]
    InvalidArgument(String),

    /// Interactive mode failed
    #[error("Interactive mode failed: {0}")]
    InteractiveModeError(String),

    /// Output operation failed
    #[error("Output operation failed: {0}")]
    OutputError(String),

    /// File operation failed
    #[error("File operation failed: {operation}")]
    FileOperation {
        operation: String,
        #[source]
        source: std::io::Error,
    },

    /// Configuration validation failed
    #[error("Configuration validation failed: {0}")]
    ValidationFailed(String),

    /// Progress indicator creation failed
    #[error("Progress indicator creation failed: {0}")]
    ProgressError(String),

    /// Terminal configuration failed
    #[error("Terminal configuration failed: {0}")]
    TerminalError(String),

    /// Transparent wrapper for library ConfigError
    #[error(transparent)]
    Config(#[from] crate::model::ConfigError),
}

impl CliError {
    /// Create a new invalid argument error
    pub fn invalid_argument<S: Into<String>>(message: S) -> Self {
        Self::InvalidArgument(message.into())
    }

    /// Create a new interactive mode error
    pub fn interactive_mode<S: Into<String>>(message: S) -> Self {
        Self::InteractiveModeError(message.into())
    }

    /// Create a new output error
    pub fn output<S: Into<String>>(message: S) -> Self {
        Self::OutputError(message.into())
    }

    /// Create a new file operation error
    pub fn file_operation<S: Into<String>>(operation: S, source: std::io::Error) -> Self {
        Self::FileOperation {
            operation: operation.into(),
            source,
        }
    }

    /// Create a new validation error
    pub fn validation<S: Into<String>>(message: S) -> Self {
        Self::ValidationFailed(message.into())
    }

    /// Create a new progress error
    pub fn progress<S: Into<String>>(message: S) -> Self {
        Self::ProgressError(message.into())
    }

    /// Create a new terminal error
    pub fn terminal<S: Into<String>>(message: S) -> Self {
        Self::TerminalError(message.into())
    }
}

/// Result type alias for CLI operations
pub type CliResult<T> = std::result::Result<T, CliError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_error_creation() {
        let error = CliError::invalid_argument("Invalid count value");
        assert!(matches!(error, CliError::InvalidArgument(_)));

        let error = CliError::interactive_mode("Failed to read user input");
        assert!(matches!(error, CliError::InteractiveModeError(_)));

        let error = CliError::output("Failed to write to file");
        assert!(matches!(error, CliError::OutputError(_)));
    }

    #[test]
    fn test_file_operation_error() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
        let error = CliError::file_operation("reading config file", io_error);
        assert!(matches!(error, CliError::FileOperation { .. }));
    }

    #[test]
    fn test_config_error_conversion() {
        let config_error = crate::model::ConfigError::validation("Invalid VLAN ID");
        let cli_error: CliError = config_error.into();
        assert!(matches!(cli_error, CliError::Config(_)));
    }
}
