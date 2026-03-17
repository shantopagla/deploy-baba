//! AWS-specific configuration types.

use serde::{Deserialize, Serialize};

/// AWS configuration.
///
/// Provides AWS-specific settings for credentials, state management,
/// and service configuration.
///
/// # Example
///
/// ```
/// use infra_types::AwsConfig;
///
/// let aws = AwsConfig {
///     profile: "deploy-baba".to_string(),
///     state_bucket_prefix: "deploy-baba-tfstate".to_string(),
///     ssm_prefix: "/deploy-baba".to_string(),
/// };
///
/// assert_eq!(aws.profile, "deploy-baba");
/// assert_eq!(aws.ssm_prefix, "/deploy-baba");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AwsConfig {
    /// AWS profile name for credential selection
    pub profile: String,

    /// S3 bucket prefix for Terraform state files
    pub state_bucket_prefix: String,

    /// AWS Systems Manager Parameter Store prefix
    pub ssm_prefix: String,
}

impl AwsConfig {
    /// Create a new AWS configuration.
    pub fn new(
        profile: impl Into<String>,
        state_bucket_prefix: impl Into<String>,
        ssm_prefix: impl Into<String>,
    ) -> Self {
        Self {
            profile: profile.into(),
            state_bucket_prefix: state_bucket_prefix.into(),
            ssm_prefix: ssm_prefix.into(),
        }
    }

    /// Get the full SSM parameter path for a given parameter name.
    ///
    /// # Example
    ///
    /// ```
    /// use infra_types::AwsConfig;
    ///
    /// let aws = AwsConfig::new("default", "tfstate", "/app");
    /// assert_eq!(aws.ssm_param_path("db_host"), "/app/db_host");
    /// ```
    pub fn ssm_param_path(&self, param: &str) -> String {
        format!("{}/{}", self.ssm_prefix, param)
    }

    /// Get the state bucket name from the prefix.
    ///
    /// # Example
    ///
    /// ```
    /// use infra_types::AwsConfig;
    ///
    /// let aws = AwsConfig::new("default", "my-app-tfstate", "/app");
    /// assert_eq!(aws.state_bucket_name(), "my-app-tfstate");
    /// ```
    pub fn state_bucket_name(&self) -> &str {
        &self.state_bucket_prefix
    }
}

impl Default for AwsConfig {
    fn default() -> Self {
        Self {
            profile: "default".to_string(),
            state_bucket_prefix: "terraform-state".to_string(),
            ssm_prefix: "/app".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aws_config_creation() {
        let aws = AwsConfig::new("my-profile", "my-tfstate", "/my-app");
        assert_eq!(aws.profile, "my-profile");
        assert_eq!(aws.state_bucket_prefix, "my-tfstate");
        assert_eq!(aws.ssm_prefix, "/my-app");
    }

    #[test]
    fn test_ssm_param_path() {
        let aws = AwsConfig::new("default", "tfstate", "/app");
        assert_eq!(aws.ssm_param_path("database_url"), "/app/database_url");
        assert_eq!(aws.ssm_param_path("api_key"), "/app/api_key");
    }

    #[test]
    fn test_state_bucket_name() {
        let aws = AwsConfig::new("default", "my-bucket", "/app");
        assert_eq!(aws.state_bucket_name(), "my-bucket");
    }

    #[test]
    fn test_aws_config_default() {
        let aws = AwsConfig::default();
        assert_eq!(aws.profile, "default");
        assert_eq!(aws.state_bucket_prefix, "terraform-state");
        assert_eq!(aws.ssm_prefix, "/app");
    }
}
