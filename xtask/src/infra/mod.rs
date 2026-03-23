//! Infrastructure management module
//!
//! Wraps Terraform operations and bootstrap procedures

use clap::Subcommand;

pub mod bootstrap;
pub mod terraform;

#[derive(Subcommand)]
pub enum InfraAction {
    /// Initialize Terraform
    Init {
        /// Working directory (defaults to "infra")
        #[arg(long)]
        dir: Option<String>,
        /// AWS profile
        #[arg(long)]
        profile: Option<String>,
    },
    /// Plan Terraform changes
    Plan {
        /// Working directory (defaults to "infra")
        #[arg(long)]
        dir: Option<String>,
        /// AWS profile
        #[arg(long)]
        profile: Option<String>,
    },
    /// Apply Terraform changes
    Apply {
        /// Working directory (defaults to "infra")
        #[arg(long)]
        dir: Option<String>,
        /// Auto-approve changes
        #[arg(long)]
        auto_approve: bool,
        /// AWS profile
        #[arg(long)]
        profile: Option<String>,
    },
    /// Destroy infrastructure
    Destroy {
        /// Working directory (defaults to "infra")
        #[arg(long)]
        dir: Option<String>,
        /// Auto-approve destruction
        #[arg(long)]
        auto_approve: bool,
        /// AWS profile
        #[arg(long)]
        profile: Option<String>,
    },
    /// Get Terraform output values
    Output {
        /// Output name
        #[arg(long)]
        name: Option<String>,
        /// Working directory (defaults to "infra")
        #[arg(long)]
        dir: Option<String>,
        /// AWS profile
        #[arg(long)]
        profile: Option<String>,
    },
    /// Bootstrap AWS account (create state bucket + DynamoDB lock table, run terraform init)
    Bootstrap {
        /// AWS profile
        #[arg(long)]
        profile: Option<String>,
        /// AWS region (default: us-east-1)
        #[arg(long)]
        region: Option<String>,
    },
}

pub async fn execute(action: InfraAction) -> anyhow::Result<()> {
    match action {
        InfraAction::Init { dir, profile } => terraform::run_terraform_init(dir, profile).await,
        InfraAction::Plan { dir, profile } => terraform::run_terraform_plan(dir, profile).await,
        InfraAction::Apply {
            dir,
            auto_approve,
            profile,
        } => terraform::run_terraform_apply(dir, auto_approve, profile).await,
        InfraAction::Destroy {
            dir,
            auto_approve,
            profile,
        } => terraform::run_terraform_destroy(dir, auto_approve, profile).await,
        InfraAction::Output { name, dir, profile } => {
            terraform::run_terraform_output(name, dir, profile).await
        }
        InfraAction::Bootstrap { profile, region } => {
            bootstrap::bootstrap_account(profile, region).await
        }
    }
}
