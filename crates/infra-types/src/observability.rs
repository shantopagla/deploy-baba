//! Observability configuration types.
//!
//! Defines logging, metrics, and alerting configurations.

use serde::{Deserialize, Serialize};

/// Observability configuration combining logging, metrics, and alerts.
///
/// # Example
///
/// ```
/// use infra_types::{ObservabilityConfig, LogLevel};
///
/// let config = ObservabilityConfig {
///     log_level: LogLevel::Info,
///     metrics: None,
///     alerts: None,
/// };
///
/// assert_eq!(config.log_level, LogLevel::Info);
/// ```
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ObservabilityConfig {
    /// Log level for the application
    #[serde(default)]
    pub log_level: LogLevel,

    /// Metrics configuration
    #[serde(default)]
    pub metrics: Option<MetricsConfig>,

    /// Alert configuration
    #[serde(default)]
    pub alerts: Option<AlertConfig>,
}

/// Log level for the application.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    /// Trace level (most verbose)
    Trace,
    /// Debug level
    Debug,
    /// Info level
    #[default]
    Info,
    /// Warn level
    Warn,
    /// Error level
    Error,
}

impl LogLevel {
    /// Get numeric representation (lower = more verbose)
    pub fn as_u8(&self) -> u8 {
        match self {
            LogLevel::Trace => 0,
            LogLevel::Debug => 1,
            LogLevel::Info => 2,
            LogLevel::Warn => 3,
            LogLevel::Error => 4,
        }
    }

    /// Check if this level includes the given level (more verbose)
    pub fn includes(&self, other: LogLevel) -> bool {
        self.as_u8() <= other.as_u8()
    }

    /// Get human-readable name
    pub fn display_name(&self) -> &'static str {
        match self {
            LogLevel::Trace => "Trace",
            LogLevel::Debug => "Debug",
            LogLevel::Info => "Info",
            LogLevel::Warn => "Warning",
            LogLevel::Error => "Error",
        }
    }
}

/// Metrics collection configuration.
///
/// # Example
///
/// ```
/// use infra_types::MetricsConfig;
///
/// let config = MetricsConfig {
///     namespace: "app-metrics".to_string(),
///     enabled: true,
/// };
///
/// assert_eq!(config.namespace, "app-metrics");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsConfig {
    /// CloudWatch namespace for metrics (AWS-specific)
    pub namespace: String,

    /// Enable metrics collection
    #[serde(default = "default_enabled")]
    pub enabled: bool,
}

fn default_enabled() -> bool {
    true
}

/// Alert configuration.
///
/// # Example
///
/// ```
/// use infra_types::AlertConfig;
///
/// let config = AlertConfig {
///     email: Some("admin@example.com".to_string()),
///     sns_topic_arn: None,
/// };
///
/// assert!(config.email.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertConfig {
    /// Email address for alert notifications
    #[serde(default)]
    pub email: Option<String>,

    /// AWS SNS Topic ARN for alert notifications
    #[serde(default)]
    pub sns_topic_arn: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_level_order() {
        // Trace (most verbose) includes everything above it
        assert!(LogLevel::Trace.includes(LogLevel::Debug));
        // Debug includes Info-level messages
        assert!(LogLevel::Debug.includes(LogLevel::Info));
        // Error does NOT include Warn messages
        assert!(!LogLevel::Error.includes(LogLevel::Warn));
    }

    #[test]
    fn test_log_level_display() {
        assert_eq!(LogLevel::Trace.display_name(), "Trace");
        assert_eq!(LogLevel::Info.display_name(), "Info");
        assert_eq!(LogLevel::Error.display_name(), "Error");
    }

    #[test]
    fn test_log_level_default() {
        assert_eq!(LogLevel::default(), LogLevel::Info);
    }

    #[test]
    fn test_metrics_config_creation() {
        let config = MetricsConfig {
            namespace: "app-metrics".to_string(),
            enabled: true,
        };
        assert_eq!(config.namespace, "app-metrics");
        assert!(config.enabled);
    }

    #[test]
    fn test_alert_config_with_email() {
        let config = AlertConfig {
            email: Some("admin@example.com".to_string()),
            sns_topic_arn: None,
        };
        assert_eq!(config.email, Some("admin@example.com".to_string()));
        assert!(config.sns_topic_arn.is_none());
    }

    #[test]
    fn test_alert_config_with_sns() {
        let config = AlertConfig {
            email: None,
            sns_topic_arn: Some("arn:aws:sns:us-east-1:123456789012:alerts".to_string()),
        };
        assert!(config.email.is_none());
        assert!(config.sns_topic_arn.is_some());
    }

    #[test]
    fn test_observability_config_default() {
        let config = ObservabilityConfig::default();
        assert_eq!(config.log_level, LogLevel::Info);
        assert!(config.metrics.is_none());
        assert!(config.alerts.is_none());
    }

    #[test]
    fn test_observability_with_metrics() {
        let metrics = MetricsConfig {
            namespace: "app".to_string(),
            enabled: true,
        };
        let config = ObservabilityConfig {
            log_level: LogLevel::Debug,
            metrics: Some(metrics),
            alerts: None,
        };
        assert_eq!(config.log_level, LogLevel::Debug);
        assert!(config.metrics.is_some());
    }

    #[test]
    fn test_observability_with_alerts() {
        let alerts = AlertConfig {
            email: Some("test@example.com".to_string()),
            sns_topic_arn: None,
        };
        let config = ObservabilityConfig {
            log_level: LogLevel::Error,
            metrics: None,
            alerts: Some(alerts),
        };
        assert!(config.alerts.is_some());
    }
}
