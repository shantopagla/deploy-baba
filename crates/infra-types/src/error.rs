//! Error types for infrastructure configuration.

use thiserror::Error;

/// Result type for infra-types operations.
pub type Result<T> = std::result::Result<T, InfraError>;

/// Infrastructure configuration errors.
#[derive(Error, Debug)]
pub enum InfraError {
    /// Configuration validation failed
    #[error("validation error: {0}")]
    Validation(String),

    /// Missing required configuration field
    #[error("missing required field: {0}")]
    MissingField(String),

    /// Invalid configuration value
    #[error("invalid {field}: {reason}")]
    InvalidValue { field: String, reason: String },

    /// I/O error during configuration loading
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    /// TOML parsing error
    #[error("toml parse error: {0}")]
    TomlParse(String),

    /// Path error
    #[error("path error: {0}")]
    Path(String),
}

impl InfraError {
    /// Create a validation error.
    pub fn validation(msg: impl Into<String>) -> Self {
        InfraError::Validation(msg.into())
    }

    /// Create a missing field error.
    pub fn missing(field: impl Into<String>) -> Self {
        InfraError::MissingField(field.into())
    }

    /// Create an invalid value error.
    pub fn invalid(field: impl Into<String>, reason: impl Into<String>) -> Self {
        InfraError::InvalidValue {
            field: field.into(),
            reason: reason.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_error() {
        let err = InfraError::validation("test validation");
        assert!(err.to_string().contains("validation error"));
    }

    #[test]
    fn test_missing_field_error() {
        let err = InfraError::missing("database_path");
        assert!(err.to_string().contains("database_path"));
    }

    #[test]
    fn test_invalid_value_error() {
        let err = InfraError::invalid("memory_mb", "must be positive");
        assert!(err.to_string().contains("memory_mb"));
        assert!(err.to_string().contains("must be positive"));
    }
}
