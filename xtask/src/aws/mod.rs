//! AWS operations module
//!
//! Provides shared AWS client setup and profile resolution

use clap::Subcommand;

pub mod ssm;
pub mod validate;

#[derive(Subcommand)]
pub enum AwsAction {
    /// Validate AWS profile and connectivity
    Validate {
        /// AWS profile name (defaults to default profile)
        #[arg(long)]
        profile: Option<String>,
    },
    /// Get SSM parameter value
    GetParameter {
        /// Parameter name
        name: String,
        /// AWS profile
        #[arg(long)]
        profile: Option<String>,
    },
    /// Set SSM parameter value
    SetParameter {
        /// Parameter name
        name: String,
        /// Parameter value
        value: String,
        /// AWS profile
        #[arg(long)]
        profile: Option<String>,
    },
}

pub async fn execute(action: AwsAction) -> anyhow::Result<()> {
    match action {
        AwsAction::Validate { profile } => validate::validate_profile(profile).await,
        AwsAction::GetParameter { name, profile } => {
            let val = ssm::get_parameter(&name, profile).await?;
            println!("{}", val);
            Ok(())
        }
        AwsAction::SetParameter {
            name,
            value,
            profile,
        } => ssm::set_parameter(&name, &value, profile).await,
    }
}

/// Create AWS SDK config with optional profile
pub async fn create_aws_config(profile: Option<String>) -> anyhow::Result<aws_config::SdkConfig> {
    let config = if let Some(p) = profile {
        aws_config::from_env()
            .profile_name(p)
            .load()
            .await
    } else {
        aws_config::load_from_env().await
    };

    Ok(config)
}
