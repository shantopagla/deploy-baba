//! Example 4: Infrastructure Types
//!
//! This example demonstrates building a complete infrastructure Stack configuration
//! that represents a production-ready deployment. It shows how to compose the various
//! infrastructure types, configure services, database backups, and observability,
//! then serialize to both JSON and TOML formats.

use infra_types::{
    AwsConfig, DeployConfig, HealthCheck, LogLevel, MetricsConfig, NetworkConfig,
    ObservabilityConfig, ProjectConfig, S3BackupConfig, ScalingConfig, ServiceConfig, SqliteConfig,
    Stack,
};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Infrastructure Types Example ===\n");

    // Build a complete Stack configuration
    println!("1. Building Infrastructure Stack");
    println!("--------------------------------");

    // Project configuration
    let project = ProjectConfig {
        name: "deploy-baba".to_string(),
        version: "0.1.0".to_string(),
        region: "us-east-1".to_string(),
    };
    println!(
        "✓ Project: {} v{} in {}",
        project.name, project.version, project.region
    );

    // Deployment configuration
    let deploy = DeployConfig {
        mode: "lambda".to_string(),
        function_name: "deploy-baba-ui".to_string(),
        runtime: "provided.al2023".to_string(),
        architecture: "arm64".to_string(),
        memory_mb: 256,
        timeout_seconds: 30,
    };
    println!(
        "✓ Deploy: {} ({}, {} memory)",
        deploy.mode, deploy.architecture, deploy.memory_mb
    );
    println!(
        "  Function: {} Runtime: {}",
        deploy.function_name, deploy.runtime
    );

    // Database configuration with S3 backup
    let database = SqliteConfig {
        path: "/mnt/db/deploy-baba.db".to_string(),
        wal_mode: true,
        backup: Some(S3BackupConfig {
            bucket: "deploy-baba-backups".to_string(),
            prefix: Some("database/".to_string()),
            retain_versions: 30,
            schedule: "rate(1 day)".to_string(),
        }),
    };
    println!("✓ Database: SQLite at {}", database.path);
    println!("  WAL Mode: {}", database.wal_mode);
    println!(
        "  Backup: S3 bucket '{}' with 30-day retention",
        database.backup.as_ref().unwrap().bucket
    );

    // Network configuration
    let mut network = NetworkConfig::new("10.0.0.0/16");
    network.subnets.push(infra_types::Subnet {
        cidr: "10.0.1.0/24".to_string(),
        availability_zone: "us-east-1a".to_string(),
        is_public: true,
    });
    println!("✓ Network: VPC {} with subnets", network.vpc_cidr);

    // Observability configuration
    let observability = ObservabilityConfig {
        log_level: LogLevel::Info,
        metrics: Some(MetricsConfig {
            namespace: "deploy-baba".to_string(),
            enabled: true,
        }),
        alerts: None,
    };
    println!(
        "✓ Observability: Log level {:?}, metrics namespace '{}'",
        observability.log_level,
        observability
            .metrics
            .as_ref()
            .map(|m| m.namespace.as_str())
            .unwrap_or("none")
    );

    // AWS configuration
    let aws = AwsConfig {
        profile: "deploy-baba".to_string(),
        state_bucket_prefix: "deploy-baba-tfstate".to_string(),
        ssm_prefix: "/deploy-baba".to_string(),
    };
    println!(
        "✓ AWS: Profile '{}' SSM prefix '{}'",
        aws.profile, aws.ssm_prefix
    );
    println!("  State bucket: {}", aws.state_bucket_name());

    // Assemble the complete stack
    let stack = Stack {
        project,
        deploy,
        database,
        observability,
        aws,
    };
    println!("\n✓ Complete Stack configured!\n");

    // Serialize to JSON
    println!("2. Serializing to JSON");
    println!("----------------------");
    let stack_json = serde_json::to_string_pretty(&stack)?;
    println!("JSON Output:\n{}\n", stack_json);

    // Serialize to TOML
    println!("3. Serializing to TOML");
    println!("----------------------");
    let stack_toml = toml::to_string_pretty(&stack)?;
    println!("TOML Output:\n{}\n", stack_toml);

    // Demonstrate builder-like patterns
    println!("4. Builder Pattern Examples");
    println!("---------------------------");

    // Example 1: Simple database configuration
    let simple_db = SqliteConfig::with_path("/var/lib/app.db").with_wal_mode(true);
    println!("Simple DB: {}", simple_db.path);
    println!("  WAL Mode: {}\n", simple_db.wal_mode);

    // Example 2: Database with backup
    let backup_config = S3BackupConfig {
        bucket: "my-backups".to_string(),
        prefix: Some("dbs/".to_string()),
        retain_versions: 14,
        schedule: "rate(12 hours)".to_string(),
    };
    let db_with_backup = SqliteConfig::with_path("/mnt/data.db").with_backup(backup_config);
    println!("DB with backup: {}", db_with_backup.path);
    println!("  Has backup: {}", db_with_backup.has_backup());
    println!(
        "  DB filename: {}\n",
        db_with_backup.filename().unwrap_or("unknown")
    );

    // Example 3: Service configuration
    let api_service = ServiceConfig::new("api", 8080)
        .with_health_check(HealthCheck {
            path: "/health".to_string(),
            interval_seconds: 30,
            healthy_threshold: 2,
        })
        .with_scaling(ScalingConfig {
            min_instances: 2,
            max_instances: 20,
            target_cpu_percent: 70,
        });
    println!(
        "API Service: {} on port {}",
        api_service.name, api_service.port
    );
    println!(
        "  Health check: {:?}",
        api_service.health_check.as_ref().map(|hc| &hc.path)
    );
    println!(
        "  Scaling: {}-{} instances",
        api_service
            .scaling
            .as_ref()
            .map(|s| s.min_instances)
            .unwrap_or(0),
        api_service
            .scaling
            .as_ref()
            .map(|s| s.max_instances)
            .unwrap_or(0)
    );

    // Show AWS helper methods
    println!("\n5. AWS Configuration Helpers");
    println!("----------------------------");
    let aws_config = AwsConfig::new("production", "prod-tfstate", "/prod");
    println!("AWS Config: {}", aws_config.profile);
    println!(
        "  SSM path for 'database_host': {}",
        aws_config.ssm_param_path("database_host")
    );
    println!("  State bucket: {}\n", aws_config.state_bucket_name());

    // Summary and serialization statistics
    println!("6. Serialization Statistics");
    println!("---------------------------");
    println!("JSON size: {} bytes", stack_json.len());
    println!("TOML size: {} bytes", stack_toml.len());
    println!(
        "Configuration keys: {}, {}, {}, {}, {}",
        if !stack.project.name.is_empty() {
            "project"
        } else {
            ""
        },
        if !stack.deploy.mode.is_empty() {
            "deploy"
        } else {
            ""
        },
        if !stack.database.path.is_empty() {
            "database"
        } else {
            ""
        },
        if stack.observability.metrics.is_some() {
            "observability"
        } else {
            "observability"
        },
        if !stack.aws.profile.is_empty() {
            "aws"
        } else {
            ""
        }
    );

    println!("\n7. Summary");
    println!("----------");
    println!("This example demonstrated:");
    println!("• Building a complete Stack with all infrastructure components");
    println!("• Project, deployment, database, observability, and AWS configuration");
    println!("• SQLite database with optional S3 backup configuration");
    println!("• Network VPC and subnet configuration");
    println!("• Serializing infrastructure configuration to JSON and TOML");
    println!("• Service configuration with health checks and auto-scaling");
    println!("• AWS-specific configuration and helper methods");
    println!("• Type-safe, serde-serializable infrastructure as code");

    Ok(())
}
