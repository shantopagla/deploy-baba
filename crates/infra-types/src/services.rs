//! Service configuration types.
//!
//! Defines service deployments, health checks, and scaling configurations.

use serde::{Deserialize, Serialize};

/// Service deployment configuration.
///
/// Represents an application service running on the infrastructure stack
/// with optional health checks and auto-scaling.
///
/// # Example
///
/// ```
/// use infra_types::{ServiceConfig, HealthCheck, ScalingConfig};
///
/// let service = ServiceConfig {
///     name: "api".to_string(),
///     port: 8080,
///     health_check: Some(HealthCheck {
///         path: "/health".to_string(),
///         interval_seconds: 30,
///         healthy_threshold: 2,
///     }),
///     scaling: Some(ScalingConfig {
///         min_instances: 1,
///         max_instances: 10,
///         target_cpu_percent: 70,
///     }),
/// };
///
/// assert_eq!(service.port, 8080);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceConfig {
    /// Service name
    pub name: String,

    /// Service port number
    pub port: u16,

    /// Health check configuration
    #[serde(default)]
    pub health_check: Option<HealthCheck>,

    /// Auto-scaling configuration
    #[serde(default)]
    pub scaling: Option<ScalingConfig>,
}

impl ServiceConfig {
    /// Create a new service configuration.
    pub fn new(name: impl Into<String>, port: u16) -> Self {
        Self {
            name: name.into(),
            port,
            health_check: None,
            scaling: None,
        }
    }

    /// Set health check configuration.
    pub fn with_health_check(mut self, health_check: HealthCheck) -> Self {
        self.health_check = Some(health_check);
        self
    }

    /// Set auto-scaling configuration.
    pub fn with_scaling(mut self, scaling: ScalingConfig) -> Self {
        self.scaling = Some(scaling);
        self
    }
}

/// Health check configuration for a service.
///
/// Configures how the orchestrator monitors service health.
///
/// # Example
///
/// ```
/// use infra_types::HealthCheck;
///
/// let hc = HealthCheck {
///     path: "/health".to_string(),
///     interval_seconds: 30,
///     healthy_threshold: 2,
/// };
///
/// assert_eq!(hc.path, "/health");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheck {
    /// HTTP path to check (e.g., "/health")
    pub path: String,

    /// Interval between health checks in seconds
    #[serde(default = "default_interval")]
    pub interval_seconds: u32,

    /// Number of consecutive successes before marking healthy
    #[serde(default = "default_healthy_threshold")]
    pub healthy_threshold: u32,
}

fn default_interval() -> u32 {
    30
}

fn default_healthy_threshold() -> u32 {
    2
}

impl HealthCheck {
    /// Create a new health check configuration.
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            interval_seconds: 30,
            healthy_threshold: 2,
        }
    }
}

/// Auto-scaling configuration for a service.
///
/// Configures horizontal scaling based on CPU metrics.
///
/// # Example
///
/// ```
/// use infra_types::ScalingConfig;
///
/// let scaling = ScalingConfig {
///     min_instances: 1,
///     max_instances: 10,
///     target_cpu_percent: 70,
/// };
///
/// assert_eq!(scaling.min_instances, 1);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScalingConfig {
    /// Minimum number of instances
    pub min_instances: u32,

    /// Maximum number of instances
    pub max_instances: u32,

    /// Target CPU utilization percentage (0-100)
    #[serde(default = "default_cpu_target")]
    pub target_cpu_percent: u32,
}

fn default_cpu_target() -> u32 {
    70
}

impl ScalingConfig {
    /// Create a new scaling configuration.
    pub fn new(min_instances: u32, max_instances: u32) -> Self {
        Self {
            min_instances,
            max_instances,
            target_cpu_percent: 70,
        }
    }

    /// Check if this configuration is valid (min <= max).
    pub fn is_valid(&self) -> bool {
        self.min_instances <= self.max_instances
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_config_creation() {
        let service = ServiceConfig::new("api", 8080);
        assert_eq!(service.name, "api");
        assert_eq!(service.port, 8080);
        assert!(service.health_check.is_none());
        assert!(service.scaling.is_none());
    }

    #[test]
    fn test_service_config_with_health_check() {
        let hc = HealthCheck::new("/health");
        let service = ServiceConfig::new("api", 8080).with_health_check(hc);

        assert!(service.health_check.is_some());
        assert_eq!(service.health_check.unwrap().path, "/health");
    }

    #[test]
    fn test_health_check_creation() {
        let hc = HealthCheck::new("/health");
        assert_eq!(hc.path, "/health");
        assert_eq!(hc.interval_seconds, 30);
        assert_eq!(hc.healthy_threshold, 2);
    }

    #[test]
    fn test_scaling_config_creation() {
        let scaling = ScalingConfig::new(1, 10);
        assert_eq!(scaling.min_instances, 1);
        assert_eq!(scaling.max_instances, 10);
        assert_eq!(scaling.target_cpu_percent, 70);
        assert!(scaling.is_valid());
    }

    #[test]
    fn test_scaling_config_validity() {
        let invalid = ScalingConfig::new(10, 1);
        assert!(!invalid.is_valid());

        let valid = ScalingConfig::new(1, 10);
        assert!(valid.is_valid());
    }
}
