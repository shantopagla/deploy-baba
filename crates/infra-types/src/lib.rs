//! Cloud-agnostic infrastructure type definitions for the deploy-baba portfolio.
//!
//! This crate provides TOML-serializable configuration types for infrastructure stacks,
//! supporting Lambda/ECS deployment, SQLite databases with S3 backup, and observability.
//!
//! # Features
//!
//! - Cloud-agnostic design (AWS, GCP, Azure, Local support)
//! - Type-safe configuration with serde (de)serialization
//! - SQLite-only database support with optional S3 backups
//! - Deploy modes: Lambda, ECS Fargate Spot
//! - Comprehensive error handling via `thiserror`
//!
//! # Example
//!
//! Deserialize a stack from TOML:
//!
//! ```ignore
//! use infra_types::Stack;
//!
//! let toml_str = r#"
//! [project]
//! name = "deploy-baba"
//! version = "0.1.0"
//! region = "us-east-1"
//!
//! [deploy]
//! mode = "lambda"
//! function_name = "deploy-baba-ui"
//! runtime = "provided.al2023"
//! architecture = "arm64"
//! memory_mb = 256
//! timeout_seconds = 30
//!
//! [database]
//! path = "/mnt/db/deploy-baba.db"
//! wal_mode = true
//!
//! [observability]
//! log_level = "info"
//! cloudwatch_namespace = "deploy-baba"
//!
//! [aws]
//! profile = "deploy-baba"
//! state_bucket_prefix = "deploy-baba-tfstate"
//! ssm_prefix = "/deploy-baba"
//! "#;
//!
//! let stack: Stack = toml::from_str(toml_str).expect("valid stack.toml");
//! assert_eq!(stack.project.name, "deploy-baba");
//! ```

pub mod aws;
pub mod database;
pub mod error;
pub mod network;
pub mod observability;
pub mod services;
pub mod stack;

// Re-export public types
pub use aws::AwsConfig;
pub use database::{S3BackupConfig, SqliteConfig};
pub use error::{InfraError, Result};
pub use network::{EgressRule, IngressRule, NetworkConfig, SecurityGroup, Subnet};
pub use observability::{AlertConfig, LogLevel, MetricsConfig, ObservabilityConfig};
pub use services::{HealthCheck, ScalingConfig, ServiceConfig};
pub use stack::{DeployConfig, DeployMode, Environment, ProjectConfig, Provider, Stack};
