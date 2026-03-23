//! Database operations module
//!
//! Handles backup and restore of SQLite databases

use clap::Subcommand;

pub mod backup;
pub mod restore;

#[derive(Subcommand)]
pub enum DatabaseAction {
    /// Backup database to S3
    Backup {
        /// Database file path
        #[arg(long)]
        path: Option<String>,
        /// AWS profile
        #[arg(long)]
        profile: Option<String>,
    },
    /// Restore database from S3
    Restore {
        /// Backup version to restore
        #[arg(long)]
        version: Option<String>,
        /// Target database path
        #[arg(long)]
        path: Option<String>,
        /// AWS profile
        #[arg(long)]
        profile: Option<String>,
    },
    /// List available backups
    ListBackups {
        /// AWS profile
        #[arg(long)]
        profile: Option<String>,
    },
}

pub async fn execute(action: DatabaseAction) -> anyhow::Result<()> {
    match action {
        DatabaseAction::Backup { path, profile } => backup::backup_database(path, profile).await,
        DatabaseAction::Restore {
            version,
            path,
            profile,
        } => restore::restore_database(version, path, profile).await,
        DatabaseAction::ListBackups { profile } => list_backups(profile).await,
    }
}

async fn list_backups(profile: Option<String>) -> anyhow::Result<()> {
    println!("📋 Listing available backups...");

    let config = crate::aws::create_aws_config(profile).await?;
    let client = aws_sdk_s3::Client::new(&config);

    let response = client
        .list_objects_v2()
        .bucket("deploy-baba-backups")
        .prefix("db-backups/")
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to list backups: {}", e))?;

    match response.contents {
        Some(objects) => {
            println!("✅ Available backups:");
            for obj in objects {
                if let Some(key) = obj.key {
                    println!("   - {}", key);
                }
            }
            Ok(())
        }
        None => {
            println!("ℹ️  No backups found");
            Ok(())
        }
    }
}
