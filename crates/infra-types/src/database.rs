//! Database configuration types.
//!
//! Defines SQLite-based persistence with optional S3 backup support.

use serde::{Deserialize, Serialize};

/// SQLite database configuration.
///
/// Configures SQLite as the primary database with Write-Ahead Logging (WAL)
/// support and optional S3-based backups.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SqliteConfig {
    /// Filesystem path to the SQLite database file
    pub path: String,

    /// Enable Write-Ahead Logging (WAL) mode for better concurrency
    #[serde(default = "default_wal_mode")]
    pub wal_mode: bool,

    /// Optional S3 backup configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup: Option<S3BackupConfig>,
}

fn default_wal_mode() -> bool {
    true
}

impl SqliteConfig {
    /// Create a new SQLite configuration with default settings.
    ///
    /// # Example
    ///
    /// ```
    /// use infra_types::SqliteConfig;
    ///
    /// let config = SqliteConfig::with_path("/mnt/db/app.db");
    /// assert_eq!(config.path, "/mnt/db/app.db");
    /// assert!(config.wal_mode);
    /// assert!(config.backup.is_none());
    /// ```
    pub fn with_path(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            wal_mode: true,
            backup: None,
        }
    }

    /// Enable or disable WAL mode.
    pub fn with_wal_mode(mut self, enabled: bool) -> Self {
        self.wal_mode = enabled;
        self
    }

    /// Set optional S3 backup configuration.
    pub fn with_backup(mut self, backup: S3BackupConfig) -> Self {
        self.backup = Some(backup);
        self
    }

    /// Extract the database filename from the path.
    pub fn filename(&self) -> Option<&str> {
        self.path.split('/').last()
    }

    /// Check if backup is configured.
    pub fn has_backup(&self) -> bool {
        self.backup.is_some()
    }
}

impl Default for SqliteConfig {
    fn default() -> Self {
        Self {
            path: "/mnt/db/app.db".to_string(),
            wal_mode: true,
            backup: None,
        }
    }
}

/// S3 backup configuration for SQLite database.
///
/// Automatically backs up the SQLite database to S3 on a scheduled basis
/// with configurable retention policies.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct S3BackupConfig {
    /// S3 bucket name (without s3:// prefix)
    pub bucket: String,

    /// Prefix path within the bucket for backup objects
    #[serde(default)]
    pub prefix: Option<String>,

    /// Number of backup versions to retain (default: 7)
    #[serde(default = "default_retain_versions")]
    pub retain_versions: u32,

    /// CloudWatch Events schedule expression for backup frequency
    /// Examples: "rate(1 day)", "rate(6 hours)", "cron(0 2 * * ? *)"
    #[serde(default = "default_schedule")]
    pub schedule: String,
}

fn default_retain_versions() -> u32 {
    7
}

fn default_schedule() -> String {
    "rate(1 day)".to_string()
}

impl S3BackupConfig {
    /// Create a new S3 backup configuration.
    ///
    /// # Example
    ///
    /// ```
    /// use infra_types::S3BackupConfig;
    ///
    /// let backup = S3BackupConfig::new("my-backups", "db");
    /// assert_eq!(backup.bucket, "my-backups");
    /// assert_eq!(backup.prefix, Some("db".to_string()));
    /// assert_eq!(backup.retain_versions, 7);
    /// ```
    pub fn new(bucket: impl Into<String>, prefix: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            prefix: Some(prefix.into()),
            retain_versions: 7,
            schedule: "rate(1 day)".to_string(),
        }
    }

    /// Set the number of backup versions to retain.
    pub fn with_retain_versions(mut self, count: u32) -> Self {
        self.retain_versions = count;
        self
    }

    /// Set the backup schedule using CloudWatch Events syntax.
    pub fn with_schedule(mut self, schedule: impl Into<String>) -> Self {
        self.schedule = schedule.into();
        self
    }

    /// Get the full S3 object key prefix for backups.
    ///
    /// Combines bucket path with optional prefix.
    pub fn s3_key_prefix(&self) -> String {
        match &self.prefix {
            Some(prefix) => format!("s3://{}/{}", self.bucket, prefix),
            None => format!("s3://{}", self.bucket),
        }
    }
}

impl Default for S3BackupConfig {
    fn default() -> Self {
        Self {
            bucket: "database-backups".to_string(),
            prefix: Some("sqlite/".to_string()),
            retain_versions: 7,
            schedule: "rate(1 day)".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sqlite_config_creation() {
        let config = SqliteConfig::with_path("/mnt/db/app.db");
        assert_eq!(config.path, "/mnt/db/app.db");
        assert!(config.wal_mode);
        assert!(config.backup.is_none());
    }

    #[test]
    fn test_sqlite_config_filename() {
        let config = SqliteConfig::with_path("/mnt/db/app.db");
        assert_eq!(config.filename(), Some("app.db"));
    }

    #[test]
    fn test_sqlite_config_with_backup() {
        let backup = S3BackupConfig::new("backups", "db");
        let config = SqliteConfig::with_path("/mnt/db/app.db").with_backup(backup);

        assert!(config.has_backup());
        assert!(config.backup.is_some());
    }

    #[test]
    fn test_s3_backup_config_creation() {
        let backup = S3BackupConfig::new("my-bucket", "backups/");
        assert_eq!(backup.bucket, "my-bucket");
        assert_eq!(backup.prefix, Some("backups/".to_string()));
        assert_eq!(backup.retain_versions, 7);
    }

    #[test]
    fn test_s3_backup_s3_key_prefix() {
        let backup = S3BackupConfig::new("my-bucket", "backups/");
        assert_eq!(backup.s3_key_prefix(), "s3://my-bucket/backups/");

        let backup_no_prefix = S3BackupConfig {
            bucket: "my-bucket".to_string(),
            prefix: None,
            retain_versions: 7,
            schedule: "rate(1 day)".to_string(),
        };
        assert_eq!(backup_no_prefix.s3_key_prefix(), "s3://my-bucket");
    }

    #[test]
    fn test_s3_backup_with_schedule() {
        let backup = S3BackupConfig::new("backups", "db")
            .with_schedule("cron(0 2 * * ? *)");

        assert_eq!(backup.schedule, "cron(0 2 * * ? *)");
    }

    #[test]
    fn test_sqlite_config_default() {
        let config = SqliteConfig::default();
        assert_eq!(config.path, "/mnt/db/app.db");
        assert!(config.wal_mode);
    }

    #[test]
    fn test_s3_backup_config_default() {
        let backup = S3BackupConfig::default();
        assert_eq!(backup.bucket, "database-backups");
        assert_eq!(backup.retain_versions, 7);
    }
}
