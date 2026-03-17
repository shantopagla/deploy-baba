//! AWS account bootstrap procedures
//!
//! Creates S3 state bucket, enables versioning, writes SSM sentinel parameter

use aws_sdk_s3::Client as S3Client;
use aws_sdk_ssm::Client as SsmClient;

pub async fn bootstrap_account(profile: Option<String>) -> anyhow::Result<()> {
    println!("🚀 Bootstrapping AWS account for deploy-baba...");

    let config = crate::aws::create_aws_config(profile).await?;

    // Create state bucket
    create_state_bucket(&config).await?;

    // Enable versioning
    enable_bucket_versioning(&config).await?;

    // Create/update sentinel parameter
    create_sentinel_parameter(&config).await?;

    println!("✅ AWS account bootstrap complete");
    Ok(())
}

async fn create_state_bucket(config: &aws_config::SdkConfig) -> anyhow::Result<()> {
    println!("   Creating S3 state bucket...");

    let client = S3Client::new(config);
    let bucket_name = "deploy-baba-terraform-state";

    // Check if bucket exists
    match client.head_bucket().bucket(bucket_name).send().await {
        Ok(_) => {
            println!("   ℹ️  Bucket already exists: {}", bucket_name);
            Ok(())
        }
        Err(_) => {
            // Create bucket
            client
                .create_bucket()
                .bucket(bucket_name)
                .send()
                .await
                .map_err(|e| anyhow::anyhow!("Failed to create S3 bucket: {}", e))?;

            println!("   ✅ Created S3 bucket: {}", bucket_name);
            Ok(())
        }
    }
}

async fn enable_bucket_versioning(config: &aws_config::SdkConfig) -> anyhow::Result<()> {
    println!("   Enabling S3 versioning...");

    let client = S3Client::new(config);
    let bucket_name = "deploy-baba-terraform-state";

    let versioning = aws_sdk_s3::types::VersioningConfiguration::builder()
        .status(aws_sdk_s3::types::BucketVersioningStatus::Enabled)
        .build();

    client
        .put_bucket_versioning()
        .bucket(bucket_name)
        .versioning_configuration(versioning)
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to enable versioning: {}", e))?;

    println!("   ✅ Versioning enabled");
    Ok(())
}

async fn create_sentinel_parameter(config: &aws_config::SdkConfig) -> anyhow::Result<()> {
    println!("   Creating SSM sentinel parameter...");

    let client = SsmClient::new(config);

    client
        .put_parameter()
        .name("/deploy-baba/sentinel")
        .value("bootstrap-complete")
        .overwrite(true)
        .r#type(aws_sdk_ssm::types::ParameterType::String)
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to create sentinel parameter: {}", e))?;

    println!("   ✅ Sentinel parameter created");
    Ok(())
}
