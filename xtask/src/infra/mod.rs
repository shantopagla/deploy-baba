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
        /// Working directory
        #[arg(long)]
        dir: Option<String>,
    },
    /// Plan Terraform changes
    Plan {
        /// Working directory
        #[arg(long)]
        dir: Option<String>,
    },
    /// Apply Terraform changes
    Apply {
        /// Working directory
        #[arg(long)]
        dir: Option<String>,
        /// Auto-approve changes
        #[arg(long)]
        auto_approve: bool,
    },
    /// Destroy infrastructure
    Destroy {
        /// Working directory
        #[arg(long)]
        dir: Option<String>,
        /// Auto-approve destruction
        #[arg(long)]
        auto_approve: bool,
    },
    /// Get Terraform output values
    Output {
        /// Output name
        #[arg(long)]
        name: Option<String>,
        /// Working directory
        #[arg(long)]
        dir: Option<String>,
    },
    /// Bootstrap AWS account (create state bucket, etc.)
    Bootstrap {
        /// AWS profile
        #[arg(long)]
        profile: Option<String>,
    },
}

pub async fn execute(action: InfraAction) -> anyhow::Result<()> {
    match action {
        InfraAction::Init { dir } => terraform::run_terraform_init(dir).await,
        InfraAction::Plan { dir } => terraform::run_terraform_plan(dir).await,
        InfraAction::Apply {
            dir,
            auto_approve,
        } => terraform::run_terraform_apply(dir, auto_approve).await,
        InfraAction::Destroy {
            dir,
            auto_approve,
        } => terraform::run_terraform_destroy(dir, auto_approve).await,
        InfraAction::Output { name, dir } => terraform::run_terraform_output(name, dir).await,
        InfraAction::Bootstrap { profile } => bootstrap::bootstrap_account(profile).await,
    }
}
