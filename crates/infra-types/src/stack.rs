//! Stack configuration types.
//!
//! The root-level infrastructure configuration combining environment, provider,
//! deployment mode, and all supporting infrastructure components.
//!
//! Provides ProjectConfig, DeployConfig, Environment, Provider, and DeployMode
//! types for composing a complete infrastructure Stack.

use crate::aws::AwsConfig;
use crate::database::SqliteConfig;
use crate::observability::ObservabilityConfig;
use serde::{Deserialize, Serialize};

/// Deployment environment controlling resource allocation and behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Environment {
    /// Development environment with minimal resources
    Dev,
    /// Staging environment mirroring production
    Staging,
    /// Production environment with full resources
    Prod,
}

impl Environment {
    /// Get display name for this environment
    pub fn display_name(&self) -> &'static str {
        match self {
            Environment::Dev => "Development",
            Environment::Staging => "Staging",
            Environment::Prod => "Production",
        }
    }

    /// Check if this is a production environment
    pub fn is_production(&self) -> bool {
        matches!(self, Environment::Prod)
    }

    /// Check if this is a non-production environment
    pub fn is_non_production(&self) -> bool {
        !self.is_production()
    }
}

/// Cloud provider selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Provider {
    /// Amazon Web Services
    Aws,
    /// Google Cloud Platform
    Gcp,
    /// Microsoft Azure
    Azure,
    /// Local development environment
    Local,
}

impl Provider {
    /// Get display name for this provider
    pub fn display_name(&self) -> &'static str {
        match self {
            Provider::Aws => "AWS",
            Provider::Gcp => "Google Cloud",
            Provider::Azure => "Azure",
            Provider::Local => "Local",
        }
    }

    /// Check if this is a cloud provider (vs local)
    pub fn is_cloud(&self) -> bool {
        !matches!(self, Provider::Local)
    }
}

/// Compute deployment mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DeployMode {
    /// AWS Lambda serverless functions
    Lambda,
    /// ECS Fargate with Spot instances for cost optimization
    EcsFargateSpot,
}

impl DeployMode {
    /// Get display name for this deployment mode
    pub fn display_name(&self) -> &'static str {
        match self {
            DeployMode::Lambda => "AWS Lambda",
            DeployMode::EcsFargateSpot => "ECS Fargate Spot",
        }
    }

    /// Check if this is a serverless deployment
    pub fn is_serverless(&self) -> bool {
        matches!(self, DeployMode::Lambda)
    }
}

/// Project configuration.
///
/// Contains project metadata and cloud region information.
///
/// # Example
///
/// ```
/// use infra_types::ProjectConfig;
///
/// let project = ProjectConfig {
///     name: "deploy-baba".to_string(),
///     version: "0.1.0".to_string(),
///     region: "us-east-1".to_string(),
/// };
///
/// assert_eq!(project.name, "deploy-baba");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    /// Project/application name
    pub name: String,

    /// Project version (semver)
    pub version: String,

    /// Primary cloud region or location
    pub region: String,
}

impl ProjectConfig {
    /// Create a new project configuration.
    pub fn new(
        name: impl Into<String>,
        version: impl Into<String>,
        region: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            region: region.into(),
        }
    }
}

/// Deployment configuration.
///
/// Specifies compute deployment mode, function settings, and runtime configuration.
///
/// # Example
///
/// ```
/// use infra_types::DeployConfig;
///
/// let deploy = DeployConfig {
///     mode: "lambda".to_string(),
///     function_name: "deploy-baba-ui".to_string(),
///     runtime: "provided.al2023".to_string(),
///     architecture: "arm64".to_string(),
///     memory_mb: 256,
///     timeout_seconds: 30,
/// };
///
/// assert_eq!(deploy.memory_mb, 256);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployConfig {
    /// Deployment mode: "lambda" or "ecs-fargate-spot"
    pub mode: String,

    /// Lambda function name
    pub function_name: String,

    /// Runtime identifier (e.g., "provided.al2023")
    pub runtime: String,

    /// Architecture: "arm64" or "x86_64"
    pub architecture: String,

    /// Memory allocation in MB
    pub memory_mb: u32,

    /// Timeout in seconds
    pub timeout_seconds: u32,
}

impl DeployConfig {
    /// Check if this is a Lambda deployment.
    pub fn is_lambda(&self) -> bool {
        self.mode.to_lowercase() == "lambda"
    }

    /// Check if this is an ECS Fargate Spot deployment.
    pub fn is_ecs_fargate_spot(&self) -> bool {
        self.mode.to_lowercase() == "ecs-fargate-spot"
    }

    /// Check if this is ARM architecture.
    pub fn is_arm64(&self) -> bool {
        self.architecture.to_lowercase() == "arm64"
    }
}

/// Root infrastructure stack configuration.
///
/// Composes all infrastructure components including project, deployment,
/// database, and observability settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stack {
    /// Project configuration
    #[serde(flatten)]
    pub project: ProjectConfig,

    /// Deployment configuration
    pub deploy: DeployConfig,

    /// Database configuration (SQLite with optional S3 backup)
    pub database: SqliteConfig,

    /// Observability configuration (logging, metrics, alerts)
    pub observability: ObservabilityConfig,

    /// AWS configuration
    pub aws: AwsConfig,
}

impl Stack {
    /// Create a new stack with required components.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use infra_types::{Stack, ProjectConfig, DeployConfig, SqliteConfig,
    ///                   ObservabilityConfig, AwsConfig};
    ///
    /// let project = ProjectConfig::new("my-app", "0.1.0", "us-east-1");
    /// let deploy = DeployConfig {
    ///     mode: "lambda".to_string(),
    ///     function_name: "my-func".to_string(),
    ///     runtime: "provided.al2023".to_string(),
    ///     architecture: "arm64".to_string(),
    ///     memory_mb: 256,
    ///     timeout_seconds: 30,
    /// };
    ///
    /// let stack = Stack {
    ///     project,
    ///     deploy,
    ///     database: SqliteConfig::default(),
    ///     observability: ObservabilityConfig::default(),
    ///     aws: AwsConfig::default(),
    /// };
    ///
    /// assert_eq!(stack.project.name, "my-app");
    /// ```
    pub fn identifier(&self) -> String {
        format!("{}-{}", self.project.name, self.project.region)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_environment_display() {
        assert_eq!(Environment::Dev.display_name(), "Development");
        assert_eq!(Environment::Staging.display_name(), "Staging");
        assert_eq!(Environment::Prod.display_name(), "Production");
    }

    #[test]
    fn test_environment_is_production() {
        assert!(!Environment::Dev.is_production());
        assert!(!Environment::Staging.is_production());
        assert!(Environment::Prod.is_production());
    }

    #[test]
    fn test_provider_is_cloud() {
        assert!(Provider::Aws.is_cloud());
        assert!(Provider::Gcp.is_cloud());
        assert!(Provider::Azure.is_cloud());
        assert!(!Provider::Local.is_cloud());
    }

    #[test]
    fn test_deploy_mode_is_serverless() {
        assert!(DeployMode::Lambda.is_serverless());
        assert!(!DeployMode::EcsFargateSpot.is_serverless());
    }

    #[test]
    fn test_project_config_creation() {
        let project = ProjectConfig::new("my-app", "0.1.0", "us-east-1");
        assert_eq!(project.name, "my-app");
        assert_eq!(project.version, "0.1.0");
        assert_eq!(project.region, "us-east-1");
    }

    #[test]
    fn test_deploy_config_is_lambda() {
        let deploy = DeployConfig {
            mode: "lambda".to_string(),
            function_name: "my-func".to_string(),
            runtime: "provided.al2023".to_string(),
            architecture: "arm64".to_string(),
            memory_mb: 256,
            timeout_seconds: 30,
        };

        assert!(deploy.is_lambda());
        assert!(!deploy.is_ecs_fargate_spot());
        assert!(deploy.is_arm64());
    }

    #[test]
    fn test_deploy_config_is_ecs() {
        let deploy = DeployConfig {
            mode: "ecs-fargate-spot".to_string(),
            function_name: "my-service".to_string(),
            runtime: "".to_string(),
            architecture: "x86_64".to_string(),
            memory_mb: 512,
            timeout_seconds: 60,
        };

        assert!(!deploy.is_lambda());
        assert!(deploy.is_ecs_fargate_spot());
        assert!(!deploy.is_arm64());
    }

    #[test]
    fn test_stack_identifier() {
        let stack = Stack {
            project: ProjectConfig::new("my-app", "0.1.0", "us-east-1"),
            deploy: DeployConfig {
                mode: "lambda".to_string(),
                function_name: "fn".to_string(),
                runtime: "provided.al2023".to_string(),
                architecture: "arm64".to_string(),
                memory_mb: 256,
                timeout_seconds: 30,
            },
            database: SqliteConfig::default(),
            observability: ObservabilityConfig::default(),
            aws: AwsConfig::default(),
        };

        assert_eq!(stack.identifier(), "my-app-us-east-1");
    }
}
